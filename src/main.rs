use clap::{Arg, Command};
use content_scraper::extract_url_content;
use serde_json::json;
use structs::GptResponse;
use tiktoken_rs::p50k_base;
use tokio::fs::{create_dir_all, OpenOptions};
use tokio::io::AsyncWriteExt;

use crate::db::get_config;

mod db;
mod structs;

#[derive(Debug)]
struct Note {
    content: String,
    file: String,
    url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("chatgpt_notes")
        .version("0.1.0")
        .author("Gustav Ekberg <krypteratkadaver@gmail.com>")
        .about("Generates notes using ChatGPT's API")
        .args([
            Arg::new("prompt")
                .long("prompt")
                .help("The input prompt for the ChatGPT API")
                .required(true),
            Arg::new("url")
                .long("url")
                .help("The relevant URL for the note being taken")
                .required(false),
            Arg::new("category")
                .long("category")
                .help("The category of the note being taken")
                .required(false),
            // Arg::new("append")
            //     .long("append")
            //     .help("Append to an existing note if it exists. Default: false")
            //     .default_value("false"),
        ])
        .get_matches();

    let config = get_config().await?;

    let api_key = if let Some(api_key) = config.api_key {
        api_key
    } else {
        println!("Please set the api_key in ~/.gptnotes.json before taking any notes");
        return Ok(());
    };

    let notes_folder = config.notes_folder;

    let relevant_url = matches.get_one::<String>("url").map(|url| url.to_string());

    let prompt = if let Some(prompt) = matches.get_one::<String>("prompt") {
        prompt.to_string()
    } else {
        println!("Please provide a prompt");
        return Ok(());
    };

    let category = matches.get_one::<String>("category");

    // Cannot use a boolean value generated from Clap, dont' know why
    // let append = if let Some(append) = matches.get_one::<String>("append") {
    //     append.eq("true")
    // } else {
    //     false
    // };

    let full_prompt = generate_prompt(&prompt, &relevant_url).await;

    let response = request_chatgpt(&full_prompt, api_key).await?;

    let note = generate_note(response, &prompt, &relevant_url);
    let file_path = save_to_md_file(note, notes_folder, category).await.unwrap();

    println!("Note saved to file {file_path}");
    Ok(())
}

async fn generate_prompt(prompt: &String, url: &Option<String>) -> String {
    let mut full_prompt =
        format!("Write summarizing notes in markdown format, explaining how to do the following: \"{prompt}\"");

    if let Some(url) = url {
        println!("Scraping url for content");
        if let Some(content) = extract_url_content(url).await.unwrap() {
            // Skip this once access to GPT4B is available
            let content = content.trim().replace("\n\n", "");
            full_prompt = format!(
                "{full_prompt}. Use this information when creating the note, if relevant: \"{content}\".");
        } else {
            println!("Could not extract content from url");
        }
    }
    full_prompt
}

async fn request_chatgpt(prompt: &str, api_key: String) -> Result<String, String> {
    let mut prompt = prompt.to_string();
    let bpe = p50k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(prompt.as_str());
    if tokens.len() > 3024 {
        println!("Prompt too long, shortening a bit");
        prompt.truncate(11000);
    }

    println!("Sending prompt to ChatGPT");

    let request_body = json!({
          "model": "gpt-3.5-turbo",
          "messages": [{
            "role": "system",
            "content": "You are a notetaking service. You only write answers in the markdown format"
        },
        {
            "content": prompt,
            "role": "user",
          }],
    });

    let client = reqwest::Client::new();
    let chatgpt_api_url = "https://api.openai.com/v1/chat/completions";

    let response: GptResponse = client
        .post(chatgpt_api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {api_key}").as_str())
        .body(request_body.to_string())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Ok(response
        .choices
        .first()
        .expect("No choices from ChatGPT")
        .message
        .content
        .clone())
}

fn generate_note(response: String, prompt: &str, url: &Option<String>) -> Note {
    let _title = response
        .lines()
        .next()
        .unwrap()
        .to_string()
        .replace('#', "")
        .trim()
        .to_string();
    let content = response;
    let file = format!("{}.md", prompt.replace(' ', "_").to_lowercase());
    Note {
        content,
        file,
        url: url.to_owned(),
    }
}

async fn save_to_md_file(
    note: Note,
    notes_folder: String,
    category: Option<&String>,
) -> std::io::Result<String> {
    let file_path = if let Some(category) = category {
        create_dir_all(format!("{notes_folder}/{category}"))
            .await
            .unwrap();
        format!("{notes_folder}/{category}/{}", note.file)
    } else {
        format!("{}/{}", notes_folder, note.file)
    };

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .await?;

    let mut content = format!("{}\n", note.content);

    if let Some(url) = note.url {
        content = format!("{content}\n\n[reference]({url})\n\n--------\n\n");
    }

    file.write_all(content.as_bytes()).await?;
    Ok(file_path)
}

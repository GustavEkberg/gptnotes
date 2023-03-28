use clap::{Arg, Command};
use db::{get_notes_folder, set_notes_folder};
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod db;
mod structs;

#[derive(Debug)]
struct Note {
    title: String,
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
                .required(false),
            Arg::new("url")
                .long("url")
                .help("The relevant URL for the note being taken")
                .required(false),
            Arg::new("category")
                .long("category")
                .help("The category of the note being taken")
                .required(false),
            Arg::new("api_key")
                .long("api_key")
                .help("Set the key to openai's api. This needs to be set before taking any notes")
                .required(false),
            Arg::new("notes_folder")
                .long("notes_folder")
                .help("Set the root folder of your notes")
                .required(false),
        ])
        .get_matches();

    let new_api_key = matches.get_one::<String>("api_key");

    let api_key = if let Some(new_api_key) = new_api_key {
        db::set_api_key(new_api_key.to_string()).await?;
        Some(new_api_key.to_string())
    } else {
        db::get_api_key().await?
    };

    if api_key.is_none() {
        println!("Please set the api_key before taking any notes");
        return Ok(());
    }

    let api_key = api_key.unwrap();

    if let Some(folder) = matches.get_one::<String>("notes_folder") {
        set_notes_folder(folder.to_string()).await?;
        return Ok(());
    };

    let notes_folder = if let Some(folder) = get_notes_folder().await? {
        folder.to_string()
    } else {
        "./".to_string()
    };

    let prompt = matches.get_one::<String>("prompt");
    let relevant_url = if let Some(url) = matches.get_one::<String>("url") {
        Some(url.to_string())
    } else {
        None
    };

    if prompt.is_none() {
        println!("Please provide a prompt");
        return Ok(());
    }
    let prompt = prompt.unwrap();

    let full_prompt = generate_prompt(&prompt, &relevant_url);
    let response = request_chatgpt(&full_prompt, api_key).await?;

    let note = generate_note(response, &prompt, &relevant_url);
    save_to_md_file(note, notes_folder).unwrap();

    Ok(())
}

fn generate_prompt(prompt: &String, url: &Option<String>) -> String {
    let mut full_prompt = format!(
        "In markdown format, write summarizing notes, explaining how to do the follow. --- {prompt}"
    );

    if let Some(url) = url {
        full_prompt = format!("--- {} Related URL: {}", prompt, url);
    }
    full_prompt
}

async fn request_chatgpt(prompt: &str, api_key: String) -> Result<String, String> {
    let request_body = json!({
          "model": "gpt-3.5-turbo",
          "messages": [{
            "role": "system",
            "content": "You are a notetaking bot. You are summarizing taking notes in the markdown format.",
        },
        {
            "content": prompt,
            "role": "user",
          }],
    });

    let client = reqwest::Client::new();
    let chatgpt_api_url = "https://api.openai.com/v1/chat/completions";

    let response = client
        .post(chatgpt_api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key).as_str())
        .body(request_body.to_string())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // Extract the generated text from the response
    let json: serde_json::Value = serde_json::from_str(&response).unwrap();
    if let Some(error) = json["error"]["message"].as_str() {
        Err(error.to_string())
    } else {
        Ok(json["choices"][0]["message"]["content"]
            .as_str()
            .expect("No response from ChatGPT")
            .to_string())
    }
}

fn generate_note(response: String, prompt: &String, url: &Option<String>) -> Note {
    let title = response
        .lines()
        .next()
        .unwrap()
        .to_string()
        .replace("# ", "");
    let content = response;
    let file = format!("{}.md", prompt.replace(" ", "_").to_lowercase());
    Note {
        title,
        content,
        file,
        url: url.to_owned(),
    }
}

fn save_to_md_file(note: Note, notes_folder: String) -> std::io::Result<()> {
    let file_path = format!("{}/{}", notes_folder, note.file);
    let path = Path::new(&file_path);
    let mut file = File::create(&path)?;

    let mut content = note.content;

    if let Some(url) = note.url {
        content = format!("{}\n\nRelated URL: [{}]({})", content, note.title, url);
    }

    file.write_all(content.as_bytes())?;
    Ok(())
}

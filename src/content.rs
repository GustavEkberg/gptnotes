use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use std::error::Error;

pub async fn extract_url_content(url: &str) -> Result<Option<String>, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let html = response.text().await?;
    let parsed_html = Html::parse_document(&html);

    let all_elements = parsed_html
        .select(&Selector::parse("main div, span, article").unwrap())
        .collect::<Vec<_>>();
    let mut max_text_len = 0;
    let mut main_content: Option<ElementRef> = None;

    for elem in &all_elements {
        let children = elem.text().collect::<Vec<_>>();
        if !children.is_empty() {
            let total_text_len: usize = children.iter().map(|child| child.len()).sum();

            if total_text_len > max_text_len {
                max_text_len = total_text_len;
                main_content = Some(*elem);
            }
        }
    }

    Ok(main_content.map(|elem| {
        elem.text()
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    }))
}

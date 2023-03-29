use reqwest::Client;
use scraper::{
    ElementRef, Html,
    Node::{self, Element, Text},
    Selector,
};
use std::error::Error;

pub async fn extract_url_content(url: &str) -> Result<Option<String>, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let html = response.text().await?;
    let parsed_html = Html::parse_document(&html);

    let all_elements = parsed_html
        .select(&Selector::parse("main, div, span, article").unwrap())
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

    let excluded = vec!["nav", "footer", "header", "script", "style", "sidebar"];

    let mut result = Vec::new();

    let element = main_content.unwrap();
    let mut stack = vec![element.clone()];

    while let Some(current) = stack.pop() {
        for child in current.children() {
            match child.value() {
                scraper::Node::Element(el) => {
                    if el.name() == "script" {
                        continue;
                    }
                    let class = el.attr("class").unwrap_or("");
                    let id = el.attr("id").unwrap_or("");

                    if excluded
                        .iter()
                        .all(|&ex| !class.contains(ex) && !id.contains(ex))
                    {
                        if let Some(el_ref) = ElementRef::wrap(child.clone()) {
                            stack.push(el_ref);
                        }
                    }
                }
                scraper::Node::Text(ref text_node) => {
                    let trimmed = text_node.trim();
                    if !trimmed.is_empty() {
                        result.push(trimmed.to_string());
                    }
                }
                _ => (),
            }
        }
    }
    let result = result.join(" ");
    if result.eq(" ") {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

use std::{fs, io::Write};

use anyhow::{Context, Result};
use dirs::home_dir;
use scraper::{Element, Html, Selector};

const URL: &str = "https://jamesclear.com/3-2-1";
const H2_TAG_TEXT: &str = "1 QUESTION FOR YOU";
const P_TAG: &str = "p";

fn get_questions(doc: &Html, sel: &Selector) -> Result<Option<String>> {
    for h2_element in doc.select(sel) {
        if h2_element.text().collect::<String>() == H2_TAG_TEXT {
            let mut p_element = h2_element
                .next_sibling_element()
                .with_context(|| format!("Couldn't find the question! {doc:#?}"))?;
            let mut questions = String::new();
            while p_element.value().name() == P_TAG {
                let text = p_element.text().collect::<String>();
                if text.starts_with("Until") {
                    break;
                } else {
                    questions.push_str(&text);
                    questions.push(' ');
                    if let Some(next_element) = p_element.next_sibling_element() {
                        p_element = next_element;
                    } else {
                        break;
                    }
                }
            }
            return Ok(Some(questions));
        }
    }
    Ok(None)
}


fn fetch_html_doc(url: &str) -> Result<String> {
    let text = reqwest::blocking::get(url)
        .with_context(|| format!("Failed to send http request: {url:?}"))?
        .error_for_status()
        .with_context(|| format!("Failed to download: {url:?}"))?
        .text()
        .context("Failed to read http response")?;
    Ok(text)
}

fn main() -> Result<()> {
    let file_name = home_dir()
        .context("Couldn't determine home directory")?
        .join("questions");
    let mut file_obj = fs::File::create(&file_name)
        .with_context(|| format!("Couldn't create file {file_name:?}"))?;
    let main_page = fetch_html_doc(URL)?;
    let main_html = Html::parse_document(&main_page);
    let a_selector = Selector::parse(r#"a[class="all-articles__news__post"]"#).unwrap();

    for element in main_html.select(&a_selector) {
        if let Some(newsletter_url) = element.value().attr("href") {
            let newsletter = fetch_html_doc(&newsletter_url)?;
            let newsletter_html = Html::parse_document(&newsletter);
            let h2_selector = Selector::parse("h2").unwrap();
            if let Some(mut questions) = get_questions(&newsletter_html, &h2_selector)? {
                questions.push('\n');
                file_obj
                    .write_all(questions.as_bytes())
                    .expect("Unable to write to file");
            }
        };
    }
    Ok(())
}

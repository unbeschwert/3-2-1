use std::{fs, io::Write};

use anyhow::{Context, Result, anyhow};
use dirs::home_dir;
use scraper::{Element, Html, Selector};

use james_clear_3_2_1::Threadpool;

const URL: &str = "https://jamesclear.com/3-2-1";
const H2_TAG_TEXT: &str = "1 QUESTION FOR YOU";
const P_TAG: &str = "p";

fn get_questions(doc: Html, sel: Selector) -> Result<Option<String>> {
    for h2_element in doc.select(&sel) {
        if h2_element.text().collect::<String>() == H2_TAG_TEXT {
            let mut p_element = h2_element
                .next_sibling_element()
                .with_context(|| format!("Couldn't find the question! {doc:#?}"))?;
            let mut question = String::new();
            while p_element.value().name() == P_TAG {
                let text = p_element.text().collect::<String>();
                if text.starts_with("Until") {
                    break;
                } else {
                    question.push_str(&text);
                    question.push(' ');
                    if let Some(next_element) = p_element.next_sibling_element() {
                        p_element = next_element;
                    } else {
                        break;
                    }
                }
            }
            return Ok(Some(question));
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
    
    let pool = Threadpool::new(4);
    let mut receivers = vec![];
    for element in main_html.select(&a_selector) {
        let href = element.value().attr("href").map(String::from);
        let receiver = pool.execute(move || {
            if let Some(newsletter_url) = href {
                let newsletter = fetch_html_doc(&newsletter_url)?;
                let newsletter_html = Html::parse_document(&newsletter);
                let h2_selector = Selector::parse("h2").unwrap();
                get_questions(newsletter_html, h2_selector)
            } else {
                Err(anyhow!("No URL found!"))
            }
        });
        receivers.push(receiver);
    }
    
    for receiver in receivers {
        match receiver.recv() {
            Ok(Ok(Some(mut question))) => {
                question.push('\n');
                if let Err(err) = file_obj.write_all(question.as_bytes()) {
                    println!("Failed to write to file: {err}");
                }
            }
            Ok(Ok(None)) => {
                // No question returned — skip silently or log
            }
            Ok(Err(err)) => {
                println!("Error in get_questions: {err}");
            }
            Err(err) => {
                println!("Failed to receive from channel: {err}");
            }
        }
    }
    Ok(())
}

use std::{fs, io::Write};

use dirs::home_dir;
use scraper::{Element, Html, Selector};

static URL: &str = "https://jamesclear.com/3-2-1";

fn get_questions(doc: &Html, sel: &Selector) -> String {
    let stopholder: &str = "p";
    let h2_text = "1 QUESTION FOR YOU";
    let mut questions = String::new();
    let mut valid: bool = true;

    for h2_element in doc.select(&sel) {
        if h2_element.text().collect::<String>() == h2_text {
            let mut p_element = match h2_element.next_sibling_element() {
                Some(x) => x,
                None => panic!("{:#?}", doc),
            };
            while valid {
                if p_element.value().name() == stopholder {
                    let text = p_element.text().collect::<String>();
                    if text.starts_with("Until") {
                        break;
                    } else {
                        questions.push_str(&text);
                        questions.push(' ');
                        p_element = match p_element.next_sibling_element() {
                            Some(x) => x,
                            None => break,
                        }
                    }
                } else {
                    valid = false;
                }
            }
        }
    }

    questions
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = home_dir().unwrap().join("questions");
    let mut file_obj = fs::File::create(file_name).expect("Unable to create file");
    let main_page = reqwest::blocking::get(URL)?.text()?;
    let main_html = Html::parse_document(&main_page);
    let a_selector = Selector::parse(r#"a[class="all-articles__news__post"]"#).unwrap();

    for element in main_html.select(&a_selector) {
        if let Some(newsletter_url) = element.value().attr("href") {
            let newsletter = reqwest::blocking::get(newsletter_url)?.text()?;
            let newsletter_html = Html::parse_document(&newsletter);
            let h2_selector = Selector::parse("h2").unwrap();
            let questions = get_questions(&newsletter_html, &h2_selector);
            if !questions.is_empty() {
                file_obj
                    .write_all(questions.as_bytes())
                    .expect("Unable to write to file");
            }
            file_obj
                .write_all("\n".as_bytes())
                .expect("Unable to write to file");
        };
    }
    Ok(())
}

use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use reqwest::Client;
use regex::Regex;

pub struct Includer;

impl Includer {
    pub fn new() -> Includer {
        Includer
    }
}

impl Preprocessor for Includer {
    fn name(&self) -> &str {
        "include-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut err = None;
        book.for_each_mut(|item| {
            process_item(item).unwrap_or_else(|e| {
                if err.is_none() {
                    err = Some(e);
                }
            })
        });
        err.map_or(Ok(book), Err)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

fn process_item(item: &mut BookItem) -> Result<(), Error> {
    if let BookItem::Chapter(ref mut ch) = item {
        ch.content = process_content(&ch.content)?;
    }
    Ok(())
}

fn process_content(content: &str) -> Result<String, Error> {
    let mut new_content = String::from(content);
    let re = Regex::new(r"!!!!include\s(.+)").unwrap();
    for cap in re.captures_iter(content) {
        if !is_url(&cap[1]) {
            continue;
        }
        let body = get_body(&cap[1])?;

        new_content = content.replace(&cap[0], &format!("```\n{}\n```", body)[..]);
    }
    Ok(new_content)
}

fn is_url(url: &str) -> bool {
    match url::Url::parse(url) {
        Ok(_) => true,
        Err(_) => false,
    }
    
}
fn get_body(url: &str) -> Result<String, Error> {
    let client = Client::new();
    let body = client.get(url).send()?.text()?;
    Ok(body)
}

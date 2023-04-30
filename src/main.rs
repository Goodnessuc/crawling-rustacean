use reqwest::blocking::get;
use scraper::{Html, Selector};

fn main() {
    let response = get("https://news.ycombinator.com").unwrap().text().unwrap();
    let html_doc = Html::parse_document(&response);
    let class_selector = Selector::parse(".titleline").unwrap();

    for element in html_doc.select(&class_selector) {
        let link_selector = Selector::parse("a").unwrap();
        for link in element.select(&link_selector) {
            if let Some(href) = link.value().attr("href") {
                println!("{}", href);
            }
        }
    }
}

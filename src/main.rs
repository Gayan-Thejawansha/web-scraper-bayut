use reqwest;
use scraper;
use scraper::Html;
use std::collections::HashMap;
use serde_json::Value;

fn main() {
    const URL:&str = "https://www.bayut.com/for-sale/apartments/dubai/"; //URL to be fetched

    let response = download_content(URL.trim());
    let document = html_parser(&response);
    
    //println!("{:?}", document);

    let results = html_title_selector (&document, "li.ef447dde");
    println!("{:?}", results.get(&1));
    print_type_of(&results);

    for (key, value) in &results {
        //println!("{}: {}", key, value);
        let filterd_document = html_fragment_parser(&value);
        //let filterd_results = html_title_selector (&filterd_document, "div._4041eb80");
        let filterd_results = html_title_selector (&filterd_document, r#"script[type="application/ld+json"]"#);
        for (_key2, value2) in &filterd_results {

            display_the_results(value2, key);

        }

 }

}

fn download_content(url: &str) -> String{
    let response = reqwest::blocking::get(url.trim()).unwrap().text().unwrap();
    return response;
}

fn html_parser(html_string:&str) -> Html{
    let html_content = scraper::Html::parse_document(&html_string);
    return html_content;
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn html_title_selector (html_content: &Html, selector_title: &str) -> HashMap<i32, String> {
    let mut content = HashMap::<i32, String>::new();
    let title_selector = scraper::Selector::parse(selector_title).unwrap();
    let titles = html_content.select(&title_selector).map(|x| {x.inner_html()});
    titles.zip(1..1000).for_each(|(item, number)| update_content_map(&mut content,number,item));

    return content;

}

fn update_content_map (map: &mut HashMap<i32, String> , key:i32, value:String){
    let _resp = map.insert(key,value);
}

fn html_fragment_parser(html_string:&str) -> Html{
    let html_content = scraper::Html::parse_fragment(&html_string);
    return html_content;
}

fn display_the_results(html_string:&str, counter:&i32) {
    //let mut json_string : JsonValue = html_string.parse()?;
    let object: Value = serde_json::from_str(html_string).unwrap();
    println!("{} - {}",counter, object["url"]);
}
use reqwest;
use scraper;
use scraper::Html;

fn main() {
    const URL:&str = "https://www.bayut.com/for-sale/apartments/dubai/"; //URL to be fetched

    let response = download_content(URL.trim());
    let document = html_parser(&response);
    print_type_of(&document);
    //println!("{:?}", document);
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
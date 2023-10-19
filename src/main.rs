extern crate chrono;

use scraper::Selector;
use scraper::Html;
use chrono::Local;

fn main() {
    let url = "https://www.coindesk.com/data/";
    let response =  reqwest::blocking::get(url).expect("Could not load url.");

    let raw_html_string = response.text().unwrap();
    let html_fragment = Html::parse_fragment(&raw_html_string);

    let coin_container_selector = Selector::parse(r#"div[class="price-liststyles__CardGrid-sc-ouhin1-1 fOYiKi"]"#)
        .unwrap();

    let coin_card_selector_pos = Selector::parse(r#"a[class="price-liststyles__ListCardWrapper-sc-ouhin1-2 hYRdsb positive"]"#).unwrap();


    let coin_card_selector_neg = Selector::parse(r#"a[class="price-liststyles__ListCardWrapper-sc-ouhin1-2 hYRdsb negative"]"#).unwrap();
    
    let coin_container = html_fragment.select(&coin_container_selector).next().unwrap();

    let current_datetime = Local::now();
    println!("Crypto prices for {}", current_datetime);

    println!("#############POSITIVE COINS#############");

    for coin_title in coin_container.select(&coin_card_selector_pos) {
        
        let coin_text = coin_title.text().collect::<Vec<_>>().join(" ");
        println!("{}", coin_text);
        println!(".............................................");
    }
    println!("#########################################");

       println!("#############NEGATIVE COINS#############");
    

    for coin_title in coin_container.select(&coin_card_selector_neg) {
        
        let coin_text = coin_title.text().collect::<Vec<_>>().join(" ");
        println!("{}", coin_text);
        println!("..............................................");
    }
    println!("#########################################");
}

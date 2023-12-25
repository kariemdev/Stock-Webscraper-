use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::time::Instant;
use std::io::{self, Write};




fn  price_finder(company : &str) -> String {
    let start = Instant::now();
    let url = format!("https://www.google.com/search?q={}+stock+price", company);
    print!("Fetching the price of {} from Google Finance...", url);
    let resp = get(&url).unwrap().text().unwrap();
    let fragment = Html::parse_document(&resp);
    let mut selector = Selector::parse(r#"div.BNeawe.iBp4i.AP7Wnd"#).unwrap();
    let mut name_selector  = Selector::parse(r#"#main > div:nth-child(7) > div > div.kCrYT > span:nth-child(1) > span"#).unwrap();
    let mut aka_selector = Selector::parse(r#"#main > div:nth-child(7) > div > div:nth-child(3) > div > div > div > div > div:nth-child(2) > div > div > div > div > span:nth-child(3)"#).unwrap();
        
    let mut  stock_price = fragment.select(&selector).next();
    let mut name = fragment.select(&name_selector).next();
    let mut aka = fragment.select(&aka_selector).next();

    let mut stock_data: Vec<&str> = Vec::new();
    let mut name_data: Vec<&str> = Vec::new();
    let mut aka_data: Vec<&str>  = Vec::new();



    
     println!("{} ggg" , company);
     match stock_price {
         Some(elem) => {
              stock_data = elem.text().collect::<Vec<_>>();
              println!("{}'s stock price is {}", company, stock_data[0]);
      } ,
          None => {println!("{}'s stock price is not available", company);
          return  "Not available".to_string();
    } 
    }
    match name {
        Some(elem) => {
             name_data = elem.text().collect::<Vec<_>>();
     } ,
         None => {println!("{}'s name is not available", company);
         name_data.push(" ");
        }}

    
    match aka {
        Some(elem) => {
             aka_data = elem.text().collect::<Vec<_>>();
     } ,
         
     None => {
                                                            
        aka_selector = Selector::parse(r#"#main > div:nth-child(6) > div > div:nth-child(3) > div > div > div > div > div:nth-child(2) > div > div > div > div > span"#).unwrap();
        aka = fragment.select(&aka_selector).next();

        
        match aka {
            Some(elem) => {
                 aka_data = elem.text().collect::<Vec<_>>();
         } ,
             None => {
                println!( "Name not availableg4gok4g");
                aka_data.push(" ");
            
        }

    }}
         
         
         }

     
     let price = stock_data[0].to_string();
     let name_final:String = name_data[0].to_string();
     let aka_final: String = aka_data[0].to_string();

     println!("{} is also known as {}", name_final, aka_final);
     println!("Time taken to fetch the price is {:?}", start.elapsed());
     price 

}


fn main() {
    while true {
    let mut input = String::new();
    print!("Enter the company name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    println!("Price of stock {} is {}", input , price_finder(&input));
}
    
    
}

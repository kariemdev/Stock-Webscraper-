use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::time::Instant;
use std::io::{self, Write};


fn  price_finder(company : &str) -> (String , String , String   ) {
    let start = Instant::now();
    let url = format!("https://www.google.com/search?q={}+stock", company);
    let resp = get(&url).unwrap().text().unwrap();
    let fragment = Html::parse_document(&resp);
    
    let mut selector = Selector::parse(r#"div.BNeawe.iBp4i.AP7Wnd"#).unwrap();
    let mut name_selector  = Selector::parse(r#".kCrYT"#).unwrap();
    let mut aka_selector = Selector::parse(r#"#main > div:nth-child(7) > div > div:nth-child(3) > div > div > div > div > div:nth-child(2) > div > div > div > div > span"#).unwrap();
        
    let mut  stock_price = fragment.select(&selector).next();
    let mut name = fragment.select(&name_selector).next();
    let mut aka = fragment.select(&aka_selector).next();

    let mut stock_data: Vec<&str> = Vec::new();
    let mut name_data: Vec<&str> = Vec::new();
    let mut aka_data: Vec<&str>  = Vec::new();
    print!("Fetching the price of {}'s stock\n", company);



    
     match stock_price {
         Some(elem) => {
              stock_data = elem.text().collect::<Vec<_>>();
      } ,
          None => {println!("{}'s stock price is not available", company);
          return  ("Not available".to_string() , "Not available".to_string() , "Not available".to_string());
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
                aka_selector = Selector::parse(r#"#main > div:nth-child(5) > div > div:nth-child(3) > div > div > div > div > div:nth-child(2) > div > div > div > div > span:nth-child(3)"#).unwrap();
                aka = fragment.select(&aka_selector).next();
                match aka{
                    Some(elem) => {
                        aka_data = elem.text().collect::<Vec<_>>();
                } ,None =>{
                    println!("{}'s name is not available", company);
                     aka_data.push(" ");
                      }
                }
        }
    }}   
         }

     
     let price = stock_data[0].to_string();
     let name_final:String = name_data[0].to_string();
     let aka_final: String = aka_data[0].to_string();

     print!("{company}'s stock price is {}\n" ,  stock_data[0]);
     print!("{} is also known as {}\n", name_final, aka_final);
     print!("Time taken to fetch the price is {:?} \n", start.elapsed());
     (name_final , price , aka_final)

}


fn main() {
    while true {
    let mut input = String::new();
    print!("Enter the company name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let (a , b , c) = price_finder(&input.trim());
    print!("Company name: {} , Stock price: {} , Stock: {} \n", a , b , c);



}
    
    
}

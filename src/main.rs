use reqwest::blocking::get;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Ip {
    ip: String,
}

fn main() {
    match get_current_date() {
        Ok(date) => println!("We've time travelled to {}!!", date),
        Err(e) => eprintln!("Oh noes, we don't know which era we're in! :( \n  {}", e),
    }
}

fn get_current_date() -> Result<String, reqwest::Error> {
    let url = "https://postman-echo.com/time/object";
    let res = get(url)?.json::<HashMap<String, i32>>()?;
    let date = res["years"].to_string();

    Ok(date)
}

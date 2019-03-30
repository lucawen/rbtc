#[macro_use] extern crate serde_derive;
use std::{env, process::exit};
use reqwest;

const API_URL: &'static str = "https://apiv2.bitcoinaverage.com/convert/global";

#[derive(Deserialize, Debug)]
struct BtcResponse {
    time: String,
    success: bool,
    price: f64
}

fn main() {
    let amount = match env::args().nth(1) {
        Some(value) => value,
        None => {
            eprintln!("Expected btc amount. Exiting.");
            exit(1);
        }
    };
    let currency = env::args().nth(2).unwrap_or(String::from("USD"));
    let client  = reqwest::Client::new();
    let request = client.get(API_URL)
        .query(&[
            ("from", "BTC"),  
            ("to", &currency),  
            ("amount", &amount),  
        ]);
    let response_result = request.send();

    let btc_response = match response_result {
        Err(e) => {
            eprintln!("Error ocurred in request conversion: {}", e);
            exit(1);
        },
        Ok(mut response) => {
            match response.json::<BtcResponse>() {
                Err(e) => {
                    eprintln!("Error ocurred in json conversion: {}", e);
                    exit(1);
            },
                Ok(json) => json,
            }
        },
    };
    if !btc_response.success {
        eprintln!("Error ocurred in conversion");
        exit(1);
    }
    println!("The conversion of {} BTC to {} is {}", amount, currency, btc_response.price);
}

#[test]
fn test_get_amount() {
    let good_input = String::from("123.456");
    assert_eq!(get_amount(good_input), 123.456);
}

#[test]
#[should_panic]
fn test_get_amount_bad() {
    let badinput = String::from("no soy btc");
    get_amount(badinput);
}

fn get_amount(input: String) -> f64 {
    input.parse().expect("Failed to parse string")
}
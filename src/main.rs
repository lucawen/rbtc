#[macro_use] extern crate serde_derive;
use std::process::exit;
use reqwest;
use structopt::StructOpt;

const API_URL: &'static str = "https://apiv2.bitcoinaverage.com/convert/global";

#[derive(Deserialize, Debug)]
struct BtcResponse {
    time: String,
    success: bool,
    price: f64
}

#[derive(Debug, StructOpt)]
#[structopt(name = "rbtc", about = "Get value of a btc value to a currency")]
struct Opt {
    /// Set amount to convert to a currency or from a currency
    amount: f64,
    /// Set the initial currency of
    #[structopt(short = "f", long = "from", default_value = "BTC")]
    from: String,
    /// Set the final currency to convert
    #[structopt(short = "t", long = "to",  default_value = "USD")]
    to: String,
    /// Verbose information not will displayed
    #[structopt(short = "s", long = "silent")]
    silent: bool,
}

fn main() {
    let opt = Opt::from_args();
    let client  = reqwest::Client::new();
    let options_req = [
        ("from", &opt.from),
        ("to", &opt.to), 
        ("amount", &opt.amount.to_string()),  
    ];
    let request = client.get(API_URL)
        .query(&options_req);
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
    if opt.silent {
        println!("{}", btc_response.price);
    } else {
        println!("{} {}", btc_response.price, &opt.to); 
    }
}

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_more;
use reqwest;
use std::process::exit;
use structopt::StructOpt;

#[derive(From, Display, Debug)]
enum RbtcError {
    ApiError,
    Reqwest(reqwest::Error),
}

const API_URL: &str = "https://apiv2.bitcoinaverage.com/convert/global";

#[derive(Debug, StructOpt)]
#[structopt(name = "rbtc", about = "Get value of a btc value to a currency")]
struct Opt {
    /// Set amount to convert to a currency or from a currency
    #[structopt(default_value = "1")]
    amount: f64,
    /// Set the initial currency of
    #[structopt(short = "f", long = "from", default_value = "BTC")]
    from: String,
    /// Set the final currency to convert
    #[structopt(short = "t", long = "to", default_value = "USD")]
    to: String,
    /// Silent information abount currency result
    #[structopt(short = "s", long = "silent")]
    silent: bool,
    /// Verbose errors
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

#[derive(Deserialize, Debug)]
struct BtcResponse {
    time: String,
    success: bool,
    price: f64,
}

fn convert_btc(amount: f64, from: &str, to: &str) -> Result<BtcResponse, RbtcError> {
    use RbtcError::*;
    let client = reqwest::Client::new();
    let request =
        client
            .get(API_URL)
            .query(&[("from", from), ("to", to), ("amount", &amount.to_string())]);
    let response_result: BtcResponse = request.send()?.json()?;

    if !response_result.success {
        return Err(ApiError);
    }

    Ok(response_result)
}

fn main() {
    let opt = Opt::from_args();

    let response = match convert_btc(opt.amount, &opt.from, &opt.to) {
        Ok(value) => value,
        Err(e) => {
            println!("A error ocurred when try to get value from api");
            if opt.verbose {
                println!("Message: {} - Details: {:?}", e, e);
            }
            exit(1);
        }
    };

    if opt.silent {
        println!("{}", response.price);
    } else {
        println!("{} {}", response.price, &opt.to);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_sucess() {
        match convert_btc(1.2, "BTC", "USD") {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_convert_error_wrong_from() {
        match convert_btc(1.2, "wrongvalue", "USD") {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_convert_error_wrong_to() {
        match convert_btc(1.2, "USD", "wrongvalue") {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}

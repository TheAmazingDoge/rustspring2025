use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use ureq;

const BTC_API_URL: &str = "https://api.coindesk.com/v1/bpi/currentprice/BTC.json";
const ETH_API_URL: &str = "https://api.coindesk.com/v1/bpi/currentprice/ETH.json";
const SP500_API_URL: &str = "https://api.example.com/sp500"; // Replace with actual S&P 500 API


pub trait Pricing {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn save_to_file(&self) -> Result<(), std::io::Error>;
}

pub fn format_price(price: f64) -> String {
    format!("{:.2}", price)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Bitcoin {
    pub price: f64,
}

impl Bitcoin {
    pub fn new(price: f64) -> Self {
        Bitcoin { price }
    }
}

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response: serde_json::Value = ureq::get(BTC_API_URL)
            .call()?
            .into_json()?;
        
        self.price = response["bpi"]["USD"]["rate_float"]
        .as_f64()
        .ok_or("Failed to parse price")?;
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), std::io::Error> {
        let file_path = "/workspaces/rustspring2025/data-fetch-rust/bitcoin_prices.txt";
        // Create the file if it doesn't exist, or append to it if it does
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;
        writeln!(file, "Bitcoin Price: ${:.2}", self.price)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ethereum {
    pub price: f64,
}
impl Ethereum {
    pub fn new(price: f64) -> Self {
        Ethereum { price }
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response: serde_json::Value = ureq::get(ETH_API_URL)
            .call()?
            .into_json()?;
        
        self.price = response["ethereum"]["usd"]
        .as_f64()
        .ok_or("Failed to parse price")?;
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), std::io::Error> {
        let file_path = "/workspaces/rustspring2025/data-fetch-rust/ethereum_prices.txt";
        // Create the file if it doesn't exist, or append to it if it does
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;
        
        writeln!(file, "Ethereum Price: ${}", self.price)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SP500 {
    pub price: f64,
}
impl SP500 {
    pub fn new(price: f64) -> Self {
        SP500 { price }
    }
}

impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response: serde_json::Value = ureq::get("https://api.example.com/sp500")
            .call()?
            .into_json()?;
        
        self.price = response["price"]
        .as_f64()
        .ok_or("Invalid price format")?;
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), std::io::Error> {
        let file_path = "/workspaces/rustspring2025/data-fetch-rust/sp500_prices.txt";
        // Create the file if it doesn't exist, or append to it if it does
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;
        
        writeln!(file, "S&P 500 Price: {}", self.price)?;
        Ok(())
    }
}

fn main() {
    let mut assets: Vec<Box<dyn Pricing>> = Vec::new();
    
    assets.push(Box::new(Bitcoin::new(0.0)));
    assets.push(Box::new(Ethereum::new(0.0)));
    assets.push(Box::new(SP500::new(0.0)));

    loop {
        for asset in &mut assets {
            if let Err(e) = asset.fetch_price() {
                eprintln!("Error fetching price: {}", e);
                continue;
            }
            if let Err(e) = asset.save_to_file() {
                eprintln!("Error saving to file: {}", e);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
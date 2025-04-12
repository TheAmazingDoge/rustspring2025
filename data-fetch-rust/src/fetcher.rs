// filepath: /data-fetch-rust/data-fetch-rust/src/fetcher.rs
use serde_json::Value;

const BTC_API_URL: &str = "https://api.coindesk.com/v1/bpi/currentprice/BTC.json";
const ETH_API_URL: &str = "https://api.coindesk.com/v1/bpi/currentprice/ETH.json";
const SP500_API_URL: &str = "https://api.example.com/sp500"; // Replace with actual S&P 500 API

/// Fetches the latest Bitcoin pricing data from the API.
pub fn fetch_bitcoin_price() -> Result<f64, Box<dyn std::error::Error>> {
    let response: Value = ureq::get(BTC_API_URL).call()?.into_json()?;
    let price = response["bpi"]["USD"]["rate_float"].as_f64().ok_or("Failed to parse Bitcoin price")?;
    Ok(price)
}

/// Fetches the latest Ethereum pricing data from the API.
pub fn fetch_ethereum_price() -> Result<f64, Box<dyn std::error::Error>> {
    let response: Value = ureq::get(ETH_API_URL).call()?.into_json()?;
    let price = response["bpi"]["USD"]["rate_float"].as_f64().ok_or("Failed to parse Ethereum price")?;
    Ok(price)
}

/// Fetches the latest S&P 500 index data from the API.
pub fn fetch_sp500_price() -> Result<f64, Box<dyn std::error::Error>> {
    let response: Value = ureq::get(SP500_API_URL).call()?.into_json()?;
    let price = response["price"].as_f64().ok_or("Failed to parse S&P 500 price")?;
    Ok(price)
}
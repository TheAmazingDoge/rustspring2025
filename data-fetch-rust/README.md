# Data Fetch Rust

This project is a Rust application that periodically fetches and records the pricing data of Bitcoin, Ethereum, and the S&P 500 index. The application runs in a loop, fetching data every 10 seconds and saving it to files.

## Project Structure

```
data-fetch-rust
├── src
│   ├── main.rs          # Entry point of the application
│   ├── assets
│   │   ├── bitcoin.rs   # Defines the Bitcoin struct and its methods
│   │   ├── ethereum.rs   # Defines the Ethereum struct and its methods
│   │   └── sp500.rs      # Defines the SP500 struct and its methods
│   ├── pricing.rs       # Defines the Pricing trait
│   ├── fetcher.rs       # Handles HTTP requests to fetch pricing data
│   └── utils.rs         # Utility functions for data formatting and file operations
├── Cargo.toml           # Project configuration and dependencies
└── README.md            # Documentation for the project
```

## Dependencies

This project uses the following dependencies:

- `ureq`: For making HTTP requests to fetch pricing data.
- `serde`: For parsing JSON data.

## Setup

1. Ensure you have Rust installed on your machine. You can install it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone the repository or download the project files.
3. Navigate to the project directory.
4. Run `cargo build` to build the project and download the necessary dependencies.

## Running the Application

To run the application, use the following command:

```
cargo run
```

The application will start fetching and saving the pricing data every 10 seconds. The data will be saved in the respective files for Bitcoin, Ethereum, and the S&P 500 index.

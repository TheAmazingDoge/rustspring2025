# Data Fetch Rust

This project is a Rust application that periodically fetches and records the pricing data of Bitcoin, Ethereum, and the S&P 500 index. The application runs in a loop, fetching data every 10 seconds and saving it to files.

## Project Structure

```
data-fetch-rust
├── src
│   ├── main.rs          # Entry point of the application as well as location of structs
├── Cargo.toml           # Project configuration and dependencies
└── README.md            # Documentation for the project
```

## Dependencies

This project uses the following dependencies:

 - ureq = { version = "2.6.1", features = ["json"] }
 - serde = { version = "1.0", features = ["derive"] }
 - serde_json = "1.0"
   
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

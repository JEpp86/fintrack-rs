use clap::{Args, Parser, Subcommand};
use fintrack_data::*;
use std::fs;

/// Command line iterface to query financial data
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the key file
    #[arg(short, long, default_value_t = String::from("./.key"))]
    key_file: String,

    /// Service to use for querying [fmp, av]
    #[arg(short, long, default_value_t = String::from("fmp"))]
    backend: String,

    /// Action to perform
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    #[command(name = "query")]
    Query(SetArgs),
    #[command(name = "quote")]
    Quote(SetArgs),
}

#[derive(Args)]
struct SetArgs {
    /// Symbol to query
    #[arg(short, long)]
    symbol: String,
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();
    let action = &args.action;
    let backend = &args.backend;

    println!("Fintrack");
    let api_key = fs::read_to_string(&args.key_file).unwrap_or_else(move |err| {
        eprintln!("Error reading keyfile at {}:\n{}", args.key_file, err);
        std::process::exit(1);
    });
    println!("API key: {}", api_key);
    println!("Backend: {}", backend);
    match action {
        Action::Query(args) => {
            println!("Query: {}", args.symbol);
            match &backend[..] {
                "fmp" => {
                    let result = FmpEndpoint::new(api_key).get_info(&args.symbol);
                    match result {
                        Ok(value) => println!("Resuls: {}", value),
                        Err(err) => println!("Error: {}", err.to_string()),
                    }
                }
                "av" => {
                    print!("AlphaVantage backend not implemented")
                }
                _ => {
                    eprintln!("Invalid backend: {}", backend);
                    std::process::exit(1);
                }
            }
        }
        Action::Quote(args) => {
            println!("Quote: {}", args.symbol);
            match &backend[..] {
                "fmp" => {
                    let result = FmpEndpoint::new(api_key).get_quote(&args.symbol);
                    match result {
                        Ok(value) => println!("Resuls: {}", value),
                        Err(err) => println!("Error: {}", err.to_string()),
                    }
                }
                "av" => {
                    print!("AlphaVantage backend not implemented")
                }
                _ => {
                    eprintln!("Invalid backend: {}", backend);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}

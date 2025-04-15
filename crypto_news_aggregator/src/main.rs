use std::io;

use crate::api::{fetch_coingecko_news, fetch_coinmarketcap_news};
use crate::error::AppError;

mod api;
mod error;
mod models;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Enter cryptocurrency name or symbol (e.g., Bitcoin, BTC):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let crypto = input.trim().to_lowercase();

    // Fetch news from CoinGecko
    let coingecko_news = fetch_coingecko_news(&crypto).await?;
    // Fetch news from CoinMarketCap (mocked as it requires API key)
    let cmc_news = fetch_coinmarketcap_news(&crypto).await?;

    // Display results
    println!("\n=== News for {} ===", crypto.to_uppercase());
    println!("\nCoinGecko News:");
    for article in coingecko_news {
        println!(
            "Title: {}\nSource: {}\nDate: {}\nSummary: {}\nLink: {}\n",
            article.title,
            article.source.unwrap_or("Unknown".to_string()),
            article.date.unwrap_or("Unknown".to_string()),
            article.summary.unwrap_or("No summary".to_string()),
            article.link.unwrap_or("No link".to_string())
        );
    }

    println!("\nCoinMarketCap News:");
    for article in cmc_news {
        println!(
            "Title: {}\nSource: {}\nDate: {}\nSummary: {}\nLink: {}\n",
            article.title,
            article.source.unwrap_or("Unknown".to_string()),
            article.date.unwrap_or("Unknown".to_string()),
            article.summary.unwrap_or("No summary".to_string()),
            article.link.unwrap_or("No link".to_string())
        );
    }

    Ok(())
}

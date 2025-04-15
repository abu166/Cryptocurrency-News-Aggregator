use crate::error::AppError;
use crate::models::NewsArticle;
use reqwest::Client;

pub async fn fetch_coingecko_news(crypto: &str) -> Result<Vec<NewsArticle>, AppError> {
    let client = Client::new();
    let url = "https://api.coingecko.com/api/v3/news"; // CoinGecko news endpoint
    let response = client
        .get(url)
        .header("accept", "application/json")
        .send()
        .await;

    // Handle HTTP errors
    let response = match response {
        Ok(resp) => resp,
        Err(e) => {
            return Err(AppError::ApiError(format!(
                "CoinGecko API request failed: {}",
                e
            )));
        }
    };

    // Parse JSON
    let json: serde_json::Value = match response.json().await {
        Ok(json) => json,
        Err(e) => {
            return Err(AppError::ApiError(format!("Failed to parse CoinGecko JSON: {}", e)));
        }
    };

    // Extract articles safely
    let articles = json["data"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    let title = item["title"].as_str().map(String::from);
                    if title.as_ref().map_or(false, |t| {
                        t.to_lowercase().contains(&crypto.to_lowercase())
                    }) {
                        Some(NewsArticle {
                            title: title.unwrap_or("Untitled".to_string()),
                            source: item["source"].as_str().map(String::from),
                            date: item["updated_at"].as_str().map(String::from),
                            summary: item["description"].as_str().map(String::from),
                            link: item["url"].as_str().map(String::from),
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<NewsArticle>>()
        })
        .unwrap_or_else(|| {
            eprintln!("No 'data' array in CoinGecko response");
            vec![]
        });

    Ok(articles)
}

pub async fn fetch_coinmarketcap_news(_crypto: &str) -> Result<Vec<NewsArticle>, AppError> {
    // Mocked response since CoinMarketCap requires API key
    Ok(vec![NewsArticle {
        title: format!("Sample News for {}", _crypto),
        source: Some("CoinMarketCap".to_string()),
        date: Some("2025-04-15".to_string()),
        summary: Some("This is a mocked article due to API key requirement.".to_string()),
        link: Some("https://coinmarketcap.com".to_string()),
    }])
}

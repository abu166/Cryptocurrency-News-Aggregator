use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsArticle {
    pub title: String,
    pub source: Option<String>,
    pub date: Option<String>,
    pub summary: Option<String>,
    pub link: Option<String>,
}

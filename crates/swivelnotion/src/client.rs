use crate::error::NotionError;
use reqwest::blocking::Client;
use serde_json::Value;
use std::env;

const BASE_URL: &str = "https://api.notion.com/v1";
const NOTION_VERSION: &str = "2026-03-11";

pub struct NotionClient {
    client: Client,
    api_key: String,
}

impl NotionClient {
    pub fn from_env() -> Result<Self, NotionError> {
        let api_key = env::var("NOTION_API_KEY").map_err(|_| NotionError::MissingApiKey)?;
        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    fn get_json(&self, url: &str) -> Result<Value, NotionError> {
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Notion-Version", NOTION_VERSION)
            .send()?;

        let status = response.status();
        let text = response.text()?;

        if !status.is_success() {
            return Err(NotionError::HttpStatus { status, body: text });
        }

        Ok(serde_json::from_str(&text)?)
    }

    pub fn get_page_raw(&self, page_id: &str) -> Result<Value, NotionError> {
        self.get_json(&format!("{BASE_URL}/pages/{page_id}"))
    }

    pub fn get_database_raw(&self, database_id: &str) -> Result<Value, NotionError> {
        self.get_json(&format!("{BASE_URL}/databases/{database_id}"))
    }

    pub fn get_data_source_raw(&self, data_source_id: &str) -> Result<Value, NotionError> {
        self.get_json(&format!("{BASE_URL}/data_sources/{data_source_id}"))
    }
}

use crate::error::NotionError;
use crate::types::{NotionBlock, NotionBlockList, NotionPage};
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

    pub fn get_block_children_raw(
        &self,
        block_id: &str,
        start_cursor: Option<&str>,
        page_size: usize,
    ) -> Result<Value, NotionError> {
        let mut url = format!("{BASE_URL}/blocks/{block_id}/children?page_size={page_size}");

        if let Some(cursor) = start_cursor {
            url.push_str("&start_cursor=");
            url.push_str(cursor);
        }

        self.get_json(&url)
    }

    pub fn get_page_typed(&self, page_id: &str) -> Result<NotionPage, NotionError> {
        let value = self.get_page_raw(page_id)?;
        Ok(serde_json::from_value(value)?)
    }

    pub fn get_block_children_typed(
        &self,
        block_id: &str,
        start_cursor: Option<&str>,
        page_size: usize,
    ) -> Result<NotionBlockList, NotionError> {
        let value = self.get_block_children_raw(block_id, start_cursor, page_size)?;
        Ok(serde_json::from_value(value)?)
    }

    pub fn get_all_top_level_blocks(&self, page_id: &str) -> Result<Vec<NotionBlock>, NotionError> {
        let mut results = Vec::new();
        let mut cursor: Option<String> = None;

        loop {
            let page = self.get_block_children_typed(page_id, cursor.as_deref(), 100)?;
            results.extend(page.results);

            if !page.has_more {
                break;
            }

            cursor = page.next_cursor;
        }

        Ok(results)
    }

    fn get_all_child_blocks_for_block(&self, block_id: &str) -> Result<Vec<NotionBlock>, NotionError> {
        let mut results = Vec::new();
        let mut cursor: Option<String> = None;

        loop {
            let page = self.get_block_children_typed(block_id, cursor.as_deref(), 100)?;
            results.extend(page.results);

            if !page.has_more {
                break;
            }

            cursor = page.next_cursor;
        }

        Ok(results)
    }

    fn attach_children_recursive(&self, mut block: NotionBlock) -> Result<NotionBlock, NotionError> {
        if !block.has_children {
            return Ok(block);
        }

        let children = self.get_all_child_blocks_for_block(&block.id)?;
        let mut normalized_children = Vec::with_capacity(children.len());

        for child in children {
            normalized_children.push(self.attach_children_recursive(child)?);
        }

        block.children = normalized_children;
        Ok(block)
    }

    pub fn get_all_blocks_recursive(&self, page_id: &str) -> Result<Vec<NotionBlock>, NotionError> {
        let top_level = self.get_all_top_level_blocks(page_id)?;
        let mut out = Vec::with_capacity(top_level.len());

        for block in top_level {
            out.push(self.attach_children_recursive(block)?);
        }

        Ok(out)
    }
}

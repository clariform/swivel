use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionDatabase {
    pub id: String,
    pub url: String,
    #[serde(default)]
    pub title: Vec<NotionRichText>,
    #[serde(default)]
    pub description: Vec<NotionRichText>,
    #[serde(default)]
    pub properties: BTreeMap<String, NotionDatabaseProperty>,
    pub parent: Option<NotionParent>,
    pub created_time: Option<String>,
    pub last_edited_time: Option<String>,
    #[serde(default)]
    pub data_sources: Vec<NotionDatabaseDataSourceRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionDatabaseDataSourceRef {
    pub id: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionDatabaseProperty {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionPage {
    pub id: String,
    pub url: String,
    pub properties: BTreeMap<String, NotionPropertyValue>,
    pub parent: Option<NotionParent>,
    pub created_time: Option<String>,
    pub last_edited_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionParent {
    #[serde(rename = "type")]
    pub kind: String,
    pub page_id: Option<String>,
    pub database_id: Option<String>,
    pub data_source_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionPropertyValue {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,

    #[serde(default)]
    pub title: Option<Vec<NotionRichText>>,
    #[serde(default)]
    pub rich_text: Option<Vec<NotionRichText>>,
    #[serde(default)]
    pub select: Option<NotionSelectOption>,
    #[serde(default)]
    pub multi_select: Option<Vec<NotionSelectOption>>,
    #[serde(default)]
    pub relation: Option<Vec<NotionRelationRef>>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub date: Option<serde_json::Value>,
    #[serde(default)]
    pub number: Option<f64>,
    #[serde(default)]
    pub checkbox: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionSelectOption {
    pub id: Option<String>,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionRelationRef {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionRichText {
    pub plain_text: String,
    pub href: Option<String>,
    #[serde(default)]
    pub annotations: Option<NotionAnnotations>,
    #[serde(default)]
    pub text: Option<NotionTextContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionAnnotations {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub strikethrough: Option<bool>,
    pub underline: Option<bool>,
    pub code: Option<bool>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionTextContent {
    pub content: String,
    pub link: Option<NotionLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionLink {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionPageQueryResult {
    pub results: Vec<NotionPage>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionBlockList {
    pub results: Vec<NotionBlock>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotionBlock {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub has_children: bool,

    #[serde(default)]
    pub paragraph: Option<NotionRichTextBlock>,
    #[serde(default)]
    pub heading_1: Option<NotionHeadingBlock>,
    #[serde(default)]
    pub heading_2: Option<NotionHeadingBlock>,
    #[serde(default)]
    pub heading_3: Option<NotionHeadingBlock>,
    #[serde(default)]
    pub heading_4: Option<NotionHeadingBlock>,
    #[serde(default)]
    pub bulleted_list_item: Option<NotionRichTextBlock>,
    #[serde(default)]
    pub numbered_list_item: Option<NotionRichTextBlock>,
    #[serde(default)]
    pub quote: Option<NotionRichTextBlock>,
    #[serde(default)]
    pub to_do: Option<NotionToDoBlock>,
    #[serde(default)]
    pub toggle: Option<NotionRichTextBlock>,
    #[serde(default)]
    pub code: Option<NotionCodeBlock>,
    #[serde(default)]
    pub callout: Option<NotionCalloutBlock>,
    #[serde(default)]
    pub equation: Option<NotionEquationBlock>,
    #[serde(default)]
    pub child_page: Option<NotionChildPageBlock>,
    #[serde(default)]
    pub table: Option<NotionTableBlock>,
    #[serde(default)]
    pub table_row: Option<NotionTableRowBlock>,

    #[serde(skip)]
    pub children: Vec<NotionBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionRichTextBlock {
    #[serde(default)]
    pub rich_text: Vec<NotionRichText>,
    pub color: Option<String>,
    #[serde(default)]
    pub is_toggleable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionHeadingBlock {
    #[serde(default)]
    pub rich_text: Vec<NotionRichText>,
    pub color: Option<String>,
    #[serde(default)]
    pub is_toggleable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionToDoBlock {
    #[serde(default)]
    pub rich_text: Vec<NotionRichText>,
    pub checked: bool,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionCodeBlock {
    #[serde(default)]
    pub rich_text: Vec<NotionRichText>,
    pub language: Option<String>,
    #[serde(default)]
    pub caption: Option<Vec<NotionRichText>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionCalloutBlock {
    #[serde(default)]
    pub rich_text: Vec<NotionRichText>,
    pub icon: Option<NotionIcon>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionIcon {
    #[serde(rename = "type")]
    pub kind: String,
    pub emoji: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionEquationBlock {
    pub expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionChildPageBlock {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionTableBlock {
    pub table_width: usize,
    pub has_column_header: bool,
    pub has_row_header: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionTableRowBlock {
    pub cells: Vec<Vec<NotionRichText>>,
}

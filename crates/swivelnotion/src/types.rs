use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionUserRef {
    pub object: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionExternalFile {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionFileObject {
    #[serde(rename = "type")]
    pub kind: String,
    pub external: Option<NotionExternalFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionIconObject {
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionIcon {
    #[serde(rename = "type")]
    pub kind: String,
    pub icon: Option<NotionIconObject>,
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
pub struct NotionTextRef {
    pub content: String,
    pub link: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionAnnotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionRichText {
    #[serde(rename = "type")]
    pub kind: String,
    pub text: Option<NotionTextRef>,
    pub annotations: Option<NotionAnnotations>,
    pub plain_text: String,
    pub href: Option<String>,
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
pub struct NotionPropertyValue {
    pub id: String,

    #[serde(rename = "type")]
    pub kind: String,

    pub title: Option<Vec<NotionRichText>>,
    pub rich_text: Option<Vec<NotionRichText>>,
    pub select: Option<NotionSelectOption>,
    pub multi_select: Option<Vec<NotionSelectOption>>,
    pub relation: Option<Vec<NotionRelationRef>>,
    pub url: Option<String>,
    pub date: Option<serde_json::Value>,
    pub number: Option<f64>,
    pub checkbox: Option<bool>,

    #[serde(default)]
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionPage {
    pub object: String,
    pub id: String,
    pub created_time: Option<String>,
    pub last_edited_time: Option<String>,
    pub created_by: Option<NotionUserRef>,
    pub last_edited_by: Option<NotionUserRef>,
    pub cover: Option<NotionFileObject>,
    pub icon: Option<NotionIcon>,
    pub parent: Option<NotionParent>,

    #[serde(default)]
    pub in_trash: bool,

    #[serde(default)]
    pub is_archived: bool,

    #[serde(default)]
    pub is_locked: bool,

    #[serde(default)]
    pub properties: BTreeMap<String, NotionPropertyValue>,

    pub url: Option<String>,
    pub public_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionBlockList {
    pub object: String,
    #[serde(default)]
    pub results: Vec<NotionBlock>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionBlockTextContainer {
    #[serde(default)]
    pub rich_text: Vec<NotionRichText>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionHeadingBlock {
    #[serde(default)]
    pub rich_text: Vec<NotionRichText>,
    pub is_toggleable: Option<bool>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionParagraphBlock {
    #[serde(default)]
    pub rich_text: Vec<NotionRichText>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionBulletedListItemBlock {
    #[serde(default)]
    pub rich_text: Vec<NotionRichText>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionBlock {
    pub object: String,
    pub id: String,
    pub parent: Option<NotionParent>,
    pub created_time: Option<String>,
    pub last_edited_time: Option<String>,
    pub created_by: Option<NotionUserRef>,
    pub last_edited_by: Option<NotionUserRef>,
    pub has_children: bool,
    #[serde(default)]
    pub in_trash: bool,

    #[serde(rename = "type")]
    pub kind: String,

    pub heading_1: Option<NotionHeadingBlock>,
    pub heading_2: Option<NotionHeadingBlock>,
    pub heading_3: Option<NotionHeadingBlock>,
    pub paragraph: Option<NotionParagraphBlock>,
    pub bulleted_list_item: Option<NotionBulletedListItemBlock>,
}

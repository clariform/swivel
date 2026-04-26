use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryRef {
    Page { id: String },
    DataSource { id: String },
    Database { id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentLineage {
    pub parent_type: Option<String>,
    pub parent_id: Option<String>,
    pub database_id: Option<String>,
    pub data_source_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value")]
pub enum PropertyValue {
    Text(String),
    Number(f64),
    Bool(bool),
    Select(String),
    MultiSelect(Vec<String>),
    Relation(Vec<String>),
    Url(String),
    Date(serde_json::Value),
    Json(serde_json::Value),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    pub properties: BTreeMap<String, PropertyValue>,
    pub relation_ids: BTreeMap<String, Vec<String>>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockNode {
    pub id: Option<String>,
    pub kind: String,
    pub text: Option<String>,
    pub children: Vec<BlockNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagDocument {
    pub id: String,
    pub source: String,
    pub source_kind: String,
    pub title: String,
    pub url: Option<String>,
    pub created_time: Option<String>,
    pub last_edited_time: Option<String>,
    pub lineage: DocumentLineage,
    pub metadata: DocumentMetadata,
    pub blocks: Vec<BlockNode>,
    pub plain_text: String,
}

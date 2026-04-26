use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentRef {
    pub kind: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedProperty {
    pub kind: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedBlock {
    pub id: Option<String>,
    pub kind: String,
    pub text: Option<String>,
    pub children: Vec<NormalizedBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedDocument {
    pub source: String,
    pub source_object: String,
    pub id: String,
    pub title: String,
    pub url: Option<String>,
    pub created_time: Option<String>,
    pub last_edited_time: Option<String>,
    pub parent: Option<ParentRef>,
    pub properties: BTreeMap<String, NormalizedProperty>,
    pub content: Vec<NormalizedBlock>,
    pub plain_text: String,
}

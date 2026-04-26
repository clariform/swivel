use std::collections::BTreeMap;

use swiveltypes::{
    BlockNode,
    DocumentLineage,
    DocumentMetadata,
    PropertyValue,
    RagDocument,
};

use crate::types::{
    NotionBlock,
    NotionPage,
    NotionPropertyValue,
    NotionRichText,
};

fn plain_text(parts: &[NotionRichText]) -> String {
    parts.iter().map(|x| x.plain_text.as_str()).collect::<String>()
}

fn title_from_page(page: &NotionPage) -> String {
    page.properties
        .values()
        .find(|prop| prop.kind == "title")
        .map(|prop| plain_text(prop.title.as_deref().unwrap_or(&[])))
        .filter(|title| !title.is_empty())
        .unwrap_or_else(|| "Untitled".to_string())
}

fn property_to_value(prop: &NotionPropertyValue) -> PropertyValue {
    match prop.kind.as_str() {
        "title" => PropertyValue::Text(plain_text(prop.title.as_deref().unwrap_or(&[]))),
        "rich_text" => PropertyValue::Text(plain_text(prop.rich_text.as_deref().unwrap_or(&[]))),
        "select" => prop
            .select
            .as_ref()
            .map(|x| PropertyValue::Select(x.name.clone()))
            .unwrap_or(PropertyValue::Null),
        "multi_select" => PropertyValue::MultiSelect(
            prop.multi_select
                .as_ref()
                .map(|xs| xs.iter().map(|x| x.name.clone()).collect())
                .unwrap_or_default(),
        ),
        "relation" => PropertyValue::Relation(
            prop.relation
                .as_ref()
                .map(|xs| xs.iter().map(|x| x.id.clone()).collect())
                .unwrap_or_default(),
        ),
        "url" => prop
            .url
            .clone()
            .map(PropertyValue::Url)
            .unwrap_or(PropertyValue::Null),
        "date" => prop
            .date
            .clone()
            .map(PropertyValue::Date)
            .unwrap_or(PropertyValue::Null),
        "number" => prop
            .number
            .map(PropertyValue::Number)
            .unwrap_or(PropertyValue::Null),
        "checkbox" => prop
            .checkbox
            .map(PropertyValue::Bool)
            .unwrap_or(PropertyValue::Null),
        _ => PropertyValue::Json(
            serde_json::to_value(prop).unwrap_or(serde_json::Value::Null),
        ),
    }
}

fn build_lineage(page: &NotionPage) -> DocumentLineage {
    let parent = page.parent.as_ref();

    DocumentLineage {
        parent_type: parent.map(|p| p.kind.clone()),
        parent_id: parent.and_then(|p| {
            p.page_id
                .clone()
                .or(p.data_source_id.clone())
                .or(p.database_id.clone())
        }),
        database_id: parent.and_then(|p| p.database_id.clone()),
        data_source_id: parent.and_then(|p| p.data_source_id.clone()),
    }
}

fn block_text(block: &NotionBlock) -> Option<String> {
    match block.kind.as_str() {
        "heading_1" => block.heading_1.as_ref().map(|b| plain_text(&b.rich_text)),
        "heading_2" => block.heading_2.as_ref().map(|b| plain_text(&b.rich_text)),
        "heading_3" => block.heading_3.as_ref().map(|b| plain_text(&b.rich_text)),
        "paragraph" => block.paragraph.as_ref().map(|b| plain_text(&b.rich_text)),
        "bulleted_list_item" => block
            .bulleted_list_item
            .as_ref()
            .map(|b| plain_text(&b.rich_text)),
        _ => None,
    }
}

fn block_to_node(block: &NotionBlock) -> BlockNode {
    BlockNode {
        id: Some(block.id.clone()),
        kind: block.kind.clone(),
        text: block_text(block),
        children: Vec::new(),
    }
}

fn blocks_to_plain_text(blocks: &[BlockNode]) -> String {
    let mut out = Vec::new();

    for block in blocks {
        if let Some(text) = &block.text {
            let text = text.trim();
            if !text.is_empty() {
                out.push(text.to_string());
            }
        }
    }

    out.join("\n\n")
}

pub fn page_to_rag_document(page: &NotionPage) -> RagDocument {
    page_and_blocks_to_rag_document(page, &[])
}

pub fn page_and_blocks_to_rag_document(page: &NotionPage, blocks: &[NotionBlock]) -> RagDocument {
    let mut properties = BTreeMap::new();
    let mut relation_ids = BTreeMap::new();
    let mut tags = Vec::new();

    for (name, prop) in &page.properties {
        let value = property_to_value(prop);

        if let PropertyValue::Relation(ids) = &value {
            relation_ids.insert(name.clone(), ids.clone());
        }

        if let PropertyValue::Select(v) = &value {
            tags.push(v.clone());
        }

        if let PropertyValue::MultiSelect(vs) = &value {
            tags.extend(vs.clone());
        }

        properties.insert(name.clone(), value);
    }

    let normalized_blocks: Vec<BlockNode> = blocks.iter().map(block_to_node).collect();
    let plain_text = blocks_to_plain_text(&normalized_blocks);

    RagDocument {
        id: page.id.clone(),
        source: "notion".to_string(),
        source_kind: "page".to_string(),
        title: title_from_page(page),
        url: page.url.clone(),
        created_time: page.created_time.clone(),
        last_edited_time: page.last_edited_time.clone(),
        lineage: build_lineage(page),
        metadata: DocumentMetadata {
            properties,
            relation_ids,
            tags,
        },
        blocks: normalized_blocks,
        plain_text,
    }
}

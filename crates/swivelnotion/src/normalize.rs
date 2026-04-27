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
    NotionTableRowBlock,
};

fn plain_text(parts: &[NotionRichText]) -> String {
    parts.iter().map(|x| x.plain_text.as_str()).collect::<String>()
}

fn rich_texts_to_plain_text(items: &[NotionRichText]) -> String {
    items
        .iter()
        .map(|x| x.plain_text.as_str())
        .collect::<Vec<_>>()
        .join("")
        .trim()
        .to_string()
}

fn table_row_to_text(row: &NotionTableRowBlock) -> String {
    let cells = row
        .cells
        .iter()
        .map(|cell| rich_texts_to_plain_text(cell))
        .collect::<Vec<_>>();

    cells.join(" | ")
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

fn metadata_json(value: serde_json::Value) -> serde_json::Value {
    value
}

fn block_to_node(block: &NotionBlock) -> BlockNode {
    let children = block.children.iter().map(block_to_node).collect::<Vec<_>>();

    match block.kind.as_str() {
        "heading_1" => BlockNode {
            id: Some(block.id.clone()),
            kind: block.kind.clone(),
            text: block.heading_1.as_ref().map(|b| plain_text(&b.rich_text)),
            metadata: metadata_json(serde_json::json!({
                "is_toggleable": block.heading_1.as_ref().and_then(|b| b.is_toggleable),
                "has_children": block.has_children
            })),
            children,
        },
        "heading_2" => BlockNode {
            id: Some(block.id.clone()),
            kind: block.kind.clone(),
            text: block.heading_2.as_ref().map(|b| plain_text(&b.rich_text)),
            metadata: metadata_json(serde_json::json!({
                "is_toggleable": block.heading_2.as_ref().and_then(|b| b.is_toggleable),
                "has_children": block.has_children
            })),
            children,
        },
        "heading_3" => BlockNode {
            id: Some(block.id.clone()),
            kind: block.kind.clone(),
            text: block.heading_3.as_ref().map(|b| plain_text(&b.rich_text)),
            metadata: metadata_json(serde_json::json!({
                "is_toggleable": block.heading_3.as_ref().and_then(|b| b.is_toggleable),
                "has_children": block.has_children
            })),
            children,
        },
        "heading_4" => BlockNode {
            id: Some(block.id.clone()),
            kind: block.kind.clone(),
            text: block.heading_4.as_ref().map(|b| plain_text(&b.rich_text)),
            metadata: metadata_json(serde_json::json!({
                "is_toggleable": block.heading_4.as_ref().and_then(|b| b.is_toggleable),
                "has_children": block.has_children
            })),
            children,
        },
        "paragraph" => BlockNode {
            id: Some(block.id.clone()),
            kind: block.kind.clone(),
            text: block.paragraph.as_ref().map(|b| plain_text(&b.rich_text)),
            metadata: metadata_json(serde_json::json!({
                "has_children": block.has_children
            })),
            children,
        },
        "bulleted_list_item" => BlockNode {
            id: Some(block.id.clone()),
            kind: block.kind.clone(),
            text: block
                .bulleted_list_item
                .as_ref()
                .map(|b| plain_text(&b.rich_text)),
            metadata: metadata_json(serde_json::json!({
                "has_children": block.has_children
            })),
            children,
        },
        "numbered_list_item" => BlockNode {
            id: Some(block.id.clone()),
            kind: block.kind.clone(),
            text: block
                .numbered_list_item
                .as_ref()
                .map(|b| plain_text(&b.rich_text)),
            metadata: metadata_json(serde_json::json!({
                "has_children": block.has_children
            })),
            children,
        },
        "quote" => BlockNode {
            id: Some(block.id.clone()),
            kind: block.kind.clone(),
            text: block.quote.as_ref().map(|b| plain_text(&b.rich_text)),
            metadata: metadata_json(serde_json::json!({
                "has_children": block.has_children
            })),
            children,
        },
        "to_do" => BlockNode {
            id: Some(block.id.clone()),
            kind: block.kind.clone(),
            text: block.to_do.as_ref().map(|b| plain_text(&b.rich_text)),
            metadata: metadata_json(serde_json::json!({
                "checked": block.to_do.as_ref().map(|b| b.checked).unwrap_or(false),
                "has_children": block.has_children
            })),
            children,
        },
        "toggle" => BlockNode {
            id: Some(block.id.clone()),
            kind: block.kind.clone(),
            text: block.toggle.as_ref().map(|b| plain_text(&b.rich_text)),
            metadata: metadata_json(serde_json::json!({
                "has_children": block.has_children
            })),
            children,
        },
        "code" => BlockNode {
            id: Some(block.id.clone()),
            kind: block.kind.clone(),
            text: block.code.as_ref().map(|b| plain_text(&b.rich_text)),
            metadata: metadata_json(serde_json::json!({
                "language": block.code.as_ref().and_then(|b| b.language.clone()),
                "caption": block.code.as_ref()
                    .map(|b| plain_text(b.caption.as_deref().unwrap_or(&[])))
                    .filter(|s| !s.is_empty())
            })),
            children,
        },
        "callout" => BlockNode {
            id: Some(block.id.clone()),
            kind: block.kind.clone(),
            text: block.callout.as_ref().map(|b| plain_text(&b.rich_text)),
            metadata: metadata_json(serde_json::json!({
                "has_children": block.has_children,
                "icon_type": block.callout
                    .as_ref()
                    .and_then(|b| b.icon.as_ref().map(|i| i.kind.clone()))
            })),
            children,
        },
        "divider" => BlockNode {
            id: Some(block.id.clone()),
            kind: block.kind.clone(),
            text: None,
            metadata: metadata_json(serde_json::json!({})),
            children,
        },
        "equation" => BlockNode {
            id: Some(block.id.clone()),
            kind: block.kind.clone(),
            text: block.equation.as_ref().map(|b| b.expression.clone()),
            metadata: metadata_json(serde_json::json!({
                "expression": block.equation.as_ref().map(|b| b.expression.clone())
            })),
            children,
        },
        "child_page" => {
            let title = block
                .child_page
                .as_ref()
                .map(|x| x.title.clone())
                .unwrap_or_default();

            BlockNode {
                id: Some(block.id.clone()),
                kind: "child_page".to_string(),
                text: if title.is_empty() { None } else { Some(title) },
                metadata: metadata_json(serde_json::json!({
                    "has_children": block.has_children
                })),
                children,
            }
        }
        "table" => {
            let table = block.table.as_ref();

            BlockNode {
                id: Some(block.id.clone()),
                kind: "table".to_string(),
                text: None,
                metadata: metadata_json(serde_json::json!({
                    "has_children": block.has_children,
                    "table_width": table.map(|x| x.table_width),
                    "has_column_header": table.map(|x| x.has_column_header),
                    "has_row_header": table.map(|x| x.has_row_header)
                })),
                children,
            }
        }
        "table_row" => {
            let text = block
                .table_row
                .as_ref()
                .map(table_row_to_text)
                .unwrap_or_default();

            BlockNode {
                id: Some(block.id.clone()),
                kind: "table_row".to_string(),
                text: if text.is_empty() { None } else { Some(text) },
                metadata: metadata_json(serde_json::json!({})),
                children,
            }
        }
        _ => BlockNode {
            id: Some(block.id.clone()),
            kind: format!("unsupported:{}", block.kind),
            text: None,
            metadata: metadata_json(serde_json::json!({
                "original_kind": block.kind,
                "has_children": block.has_children
            })),
            children,
        },
    }
}

fn push_plain_text_recursive(block: &BlockNode, out: &mut Vec<String>) {
    if block.kind != "divider" {
        if let Some(text) = &block.text {
            let text = text.trim();
            if !text.is_empty() {
                out.push(text.to_string());
            }
        }
    }

    for child in &block.children {
        push_plain_text_recursive(child, out);
    }
}

fn blocks_to_plain_text(blocks: &[BlockNode]) -> String {
    let mut out = Vec::new();

    for block in blocks {
        push_plain_text_recursive(block, &mut out);
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

use swiveltypes::{BlockNode, PropertyValue, RagChunk, RagDocument};

fn is_heading(kind: &str) -> bool {
    matches!(kind, "heading_1" | "heading_2" | "heading_3" | "heading_4")
}

fn is_list_item(kind: &str) -> bool {
    matches!(kind, "bulleted_list_item" | "numbered_list_item")
}

fn is_standalone(kind: &str) -> bool {
    matches!(
        kind,
        "paragraph"
            | "quote"
            | "to_do"
            | "toggle"
            | "code"
            | "callout"
            | "equation"
            | "child_page"
    )
}

fn heading_level(kind: &str) -> Option<usize> {
    match kind {
        "heading_1" => Some(1),
        "heading_2" => Some(2),
        "heading_3" => Some(3),
        "heading_4" => Some(4),
        _ => None,
    }
}

fn is_container(kind: &str) -> bool {
    matches!(kind, "callout" | "toggle" | "quote" | "to_do")
}

fn enrich_text(
    page_title: &str,
    heading_path: &[String],
    local_heading_path: &[String],
    container_path: &[String],
    chunk_kind: &str,
    body: &str,
    metadata: &serde_json::Value,
) -> String {
    let mut parts = Vec::new();

    parts.push(format!("Title: {page_title}"));

    if !heading_path.is_empty() {
        parts.push(format!("Section: {}", heading_path.join(" > ")));
    }

    if !local_heading_path.is_empty() {
        parts.push(format!("Local section: {}", local_heading_path.join(" > ")));
    }

    if !container_path.is_empty() {
        parts.push(format!("Container: {}", container_path.join(" > ")));
    }

    parts.push(format!("Chunk kind: {chunk_kind}"));

    if let Some(language) = metadata.get("language").and_then(|v| v.as_str()) {
        parts.push(format!("Language: {language}"));
    }

    parts.push(String::new());
    parts.push(body.to_string());

    parts.join("\n")
}

fn build_chunk(
    doc: &RagDocument,
    heading_path: &[String],
    local_heading_path: &[String],
    container_path: &[String],
    chunk_kind: &str,
    block_ids: Vec<String>,
    body: String,
    metadata: serde_json::Value,
    index: usize,
) -> RagChunk {
    RagChunk {
        chunk_id: format!("{}:{index:04}", doc.id),
        document_id: doc.id.clone(),
        source: doc.source.clone(),
        source_kind: doc.source_kind.clone(),
        page_title: doc.title.clone(),
        url: doc.url.clone(),
        chunk_kind: chunk_kind.to_string(),
        heading_path: heading_path.to_vec(),
        local_heading_path: local_heading_path.to_vec(),
        container_path: container_path.to_vec(),
        block_ids,
        text: enrich_text(
            &doc.title,
            heading_path,
            local_heading_path,
            container_path,
            chunk_kind,
            &body,
            &metadata,
        ),
        tags: doc.metadata.tags.clone(),
        relation_ids: doc.metadata.relation_ids.clone(),
        lineage: doc.lineage.clone(),
        metadata,
    }
}

fn block_label(block: &BlockNode) -> String {
    if let Some(text) = &block.text {
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            return format!("{}: {}", block.kind, trimmed);
        }
    }
    block.kind.clone()
}

#[derive(Clone, Debug, Default)]
struct TraversalContext {
    page_heading_path: Vec<String>,
    local_heading_path: Vec<String>,
    container_path: Vec<String>,
}

struct ChunkState<'a> {
    doc: &'a RagDocument,
    chunks: Vec<RagChunk>,
    next_index: usize,
}

impl<'a> ChunkState<'a> {
    fn new(doc: &'a RagDocument) -> Self {
        Self {
            doc,
            chunks: Vec::new(),
            next_index: 0,
        }
    }

    fn push_chunk(
        &mut self,
        ctx: &TraversalContext,
        chunk_kind: &str,
        block_ids: Vec<String>,
        body: String,
        metadata: serde_json::Value,
    ) {
        let chunk = build_chunk(
            self.doc,
            &ctx.page_heading_path,
            &ctx.local_heading_path,
            &ctx.container_path,
            chunk_kind,
            block_ids,
            body,
            metadata,
            self.next_index,
        );
        self.chunks.push(chunk);
        self.next_index += 1;
    }
}

fn update_heading_path(path: &mut Vec<String>, kind: &str, text: &str) {
    if let Some(level) = heading_level(kind) {
        while path.len() >= level {
            path.pop();
        }
        let text = text.trim();
        if !text.is_empty() {
            path.push(text.to_string());
        }
    }
}

fn walk_blocks(
    blocks: &[BlockNode],
    ctx: &TraversalContext,
    state: &mut ChunkState<'_>,
) {
    let mut i = 0usize;
    let mut current_ctx = ctx.clone();

    while i < blocks.len() {
        let block = &blocks[i];

        if is_heading(&block.kind) {
            if let Some(text) = &block.text {
                update_heading_path(&mut current_ctx.local_heading_path, &block.kind, text);

                if current_ctx.container_path.is_empty() {
                    update_heading_path(&mut current_ctx.page_heading_path, &block.kind, text);
                }
            }

            if !block.children.is_empty() {
                walk_blocks(&block.children, &current_ctx, state);
            }

            i += 1;
            continue;
        }

        if block.kind == "divider" {
            if !block.children.is_empty() {
                walk_blocks(&block.children, &current_ctx, state);
            }

            i += 1;
            continue;
        }

        if block.kind == "table" {
            let mut block_ids = Vec::new();
            let mut lines = Vec::new();

            for child in &block.children {
                if child.kind == "table_row" {
                    if let Some(id) = &child.id {
                        block_ids.push(id.clone());
                    }
                    if let Some(text) = &child.text {
                        let trimmed = text.trim();
                        if !trimmed.is_empty() {
                            lines.push(format!("| {trimmed} |"));
                        }
                    }
                }
            }

            state.push_chunk(
                &current_ctx,
                "table",
                block_ids,
                lines.join("\n"),
                block.metadata.clone(),
            );

            i += 1;
            continue;
        }

        if is_list_item(&block.kind) {
            let mut lines = Vec::new();
            let mut block_ids = Vec::new();
            let list_kind = block.kind.clone();

            while i < blocks.len() && blocks[i].kind == list_kind {
                let item = &blocks[i];

                if let Some(text) = &item.text {
                    let text = text.trim();
                    if !text.is_empty() {
                        let prefix = if list_kind == "numbered_list_item" {
                            "1."
                        } else {
                            "-"
                        };
                        lines.push(format!("{prefix} {text}"));
                    }
                }

                if let Some(id) = &item.id {
                    block_ids.push(id.clone());
                }

                if !item.children.is_empty() {
                    let mut nested_ctx = current_ctx.clone();
                    nested_ctx.container_path.push(block_label(item));
                    walk_blocks(&item.children, &nested_ctx, state);
                }

                i += 1;
            }

            if !lines.is_empty() {
                state.push_chunk(
                    &current_ctx,
                    "list",
                    block_ids,
                    lines.join("\n"),
                    serde_json::json!({
                        "list_kind": list_kind
                    }),
                );
            }

            continue;
        }

        if is_standalone(&block.kind) {
            let body = block.text.clone().unwrap_or_default();
            let body = body.trim().to_string();

            if !body.is_empty() || block.kind == "code" || block.kind == "equation" {
                let block_ids = block.id.clone().map(|x| vec![x]).unwrap_or_default();

                state.push_chunk(
                    &current_ctx,
                    &block.kind,
                    block_ids,
                    body,
                    block.metadata.clone(),
                );
            }

            if !block.children.is_empty() {
                let mut nested_ctx = current_ctx.clone();

                if is_container(&block.kind) {
                    nested_ctx.container_path.push(block_label(block));
                    nested_ctx.local_heading_path.clear();
                }

                walk_blocks(&block.children, &nested_ctx, state);
            }

            i += 1;
            continue;
        }

        let body = block.text.clone().unwrap_or_default();
        let body = body.trim().to_string();

        if !body.is_empty() {
            let block_ids = block.id.clone().map(|x| vec![x]).unwrap_or_default();

            state.push_chunk(
                &current_ctx,
                &block.kind,
                block_ids,
                body,
                block.metadata.clone(),
            );
        }

        if !block.children.is_empty() {
            walk_blocks(&block.children, &current_ctx, state);
        }

        i += 1;
    }
}

fn property_value_to_text(value: &PropertyValue) -> Option<String> {
    match value {
        PropertyValue::Text(s) => {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        }
        PropertyValue::Number(n) => Some(n.to_string()),
        PropertyValue::Bool(b) => Some(b.to_string()),
        PropertyValue::Select(s) => {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        }
        PropertyValue::MultiSelect(xs) => {
            if xs.is_empty() {
                None
            } else {
                Some(xs.join(", "))
            }
        }
        PropertyValue::Relation(xs) => {
            if xs.is_empty() {
                None
            } else {
                Some(xs.join(", "))
            }
        }
        PropertyValue::Url(s) => {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        }
        PropertyValue::Date(v) => {
            if v.is_null() {
                None
            } else {
                Some(v.to_string())
            }
        }
        PropertyValue::Json(v) => {
            if v.is_null() {
                None
            } else {
                Some(v.to_string())
            }
        }
        PropertyValue::Null => None,
    }
}

fn build_property_summary_chunk(doc: &RagDocument, index: usize) -> Option<RagChunk> {
    let priority_keys = [
        "Name",
        "Title",
        "Description",
        "Notes",
        "Action",
        "Command",
        "Category",
        "Tags",
        "Type",
        "Mode",
        "Platform",
        "Scope",
        "Status",
        "Tier",
        "UID",
        "Prefix",
        "Docs",
    ];

    let mut lines = Vec::new();
    let mut seen = std::collections::BTreeSet::new();

    for key in priority_keys {
        if let Some(value) = doc.metadata.properties.get(key) {
            if let Some(text) = property_value_to_text(value) {
                lines.push(format!("{key}: {text}"));
                seen.insert(key.to_string());
            }
        }
    }

    for (key, value) in &doc.metadata.properties {
        if seen.contains(key) {
            continue;
        }
        if let Some(text) = property_value_to_text(value) {
            lines.push(format!("{key}: {text}"));
        }
    }

    if lines.is_empty() {
        return None;
    }

    Some(RagChunk {
        chunk_id: format!("{}:{index:04}", doc.id),
        document_id: doc.id.clone(),
        source: doc.source.clone(),
        source_kind: doc.source_kind.clone(),
        page_title: doc.title.clone(),
        url: doc.url.clone(),
        chunk_kind: "record_summary".to_string(),
        heading_path: Vec::new(),
        local_heading_path: Vec::new(),
        container_path: Vec::new(),
        block_ids: Vec::new(),
        text: format!(
            "Title: {}\nChunk kind: record_summary\n\n{}",
            doc.title,
            lines.join("\n")
        ),
        tags: doc.metadata.tags.clone(),
        relation_ids: doc.metadata.relation_ids.clone(),
        lineage: doc.lineage.clone(),
        metadata: serde_json::json!({
            "source": "properties_fallback"
        }),
    })
}

pub fn chunk_document(doc: &RagDocument) -> Vec<RagChunk> {
    let mut state = ChunkState::new(doc);
    let ctx = TraversalContext::default();

    walk_blocks(&doc.blocks, &ctx, &mut state);

    if state.chunks.is_empty() {
        if let Some(chunk) = build_property_summary_chunk(doc, state.next_index) {
            state.chunks.push(chunk);
            state.next_index += 1;
        }
    }

    state.chunks
}

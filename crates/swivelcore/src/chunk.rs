use swiveltypes::{BlockNode, RagChunk, RagDocument};

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
        container_path: container_path.to_vec(),
        block_ids,
        text: enrich_text(
            &doc.title,
            heading_path,
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
        heading_path: &[String],
        container_path: &[String],
        chunk_kind: &str,
        block_ids: Vec<String>,
        body: String,
        metadata: serde_json::Value,
    ) {
        let chunk = build_chunk(
            self.doc,
            heading_path,
            container_path,
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

fn walk_blocks(
    blocks: &[BlockNode],
    heading_path: &mut Vec<String>,
    container_path: &mut Vec<String>,
    state: &mut ChunkState<'_>,
) {
    let mut i = 0usize;

    while i < blocks.len() {
        let block = &blocks[i];

        if is_heading(&block.kind) {
            if let Some(level) = heading_level(&block.kind) {
                while heading_path.len() >= level {
                    heading_path.pop();
                }

                if let Some(text) = &block.text {
                    let text = text.trim();
                    if !text.is_empty() {
                        heading_path.push(text.to_string());
                    }
                }
            }

            if !block.children.is_empty() {
                walk_blocks(&block.children, heading_path, container_path, state);
            }

            i += 1;
            continue;
        }

        if block.kind == "divider" {
            if !block.children.is_empty() {
                walk_blocks(&block.children, heading_path, container_path, state);
            }

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
                    let mut nested_container_path = container_path.clone();
                    nested_container_path.push(block_label(item));
                    walk_blocks(
                        &item.children,
                        heading_path,
                        &mut nested_container_path,
                        state,
                    );
                }

                i += 1;
            }

            if !lines.is_empty() {
                state.push_chunk(
                    heading_path,
                    container_path,
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
                    heading_path,
                    container_path,
                    &block.kind,
                    block_ids,
                    body,
                    block.metadata.clone(),
                );
            }

            if !block.children.is_empty() {
                let mut nested_container_path = container_path.clone();

                if is_container(&block.kind) {
                    nested_container_path.push(block_label(block));
                }

                walk_blocks(
                    &block.children,
                    heading_path,
                    &mut nested_container_path,
                    state,
                );
            }

            i += 1;
            continue;
        }

        let body = block.text.clone().unwrap_or_default();
        let body = body.trim().to_string();

        if !body.is_empty() {
            let block_ids = block.id.clone().map(|x| vec![x]).unwrap_or_default();

            state.push_chunk(
                heading_path,
                container_path,
                &block.kind,
                block_ids,
                body,
                block.metadata.clone(),
            );
        }

        if !block.children.is_empty() {
            walk_blocks(&block.children, heading_path, container_path, state);
        }

        i += 1;
    }
}

pub fn chunk_document(doc: &RagDocument) -> Vec<RagChunk> {
    let mut state = ChunkState::new(doc);
    let mut heading_path = Vec::new();
    let mut container_path = Vec::new();

    walk_blocks(&doc.blocks, &mut heading_path, &mut container_path, &mut state);

    state.chunks
}

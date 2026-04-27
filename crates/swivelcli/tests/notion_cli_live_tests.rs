use assert_cmd::Command;
use serde_json::Value;
use std::path::Path;
use tempfile::tempdir;

const DATABASE_ID: &str = "c3c6411f-0654-43cf-83ea-708bfd9097a9";
const DATA_SOURCE_ID: &str = "22dacdda-bf8b-424f-a391-0556aae54dd6";

const PAGE_IDS: [&str; 2] = [
    "4d61e57bfd12419aa7763216d02bbbbb",
    "b090a6c250114f90a3903e925f9cc97a",
];

fn compact_id(id: &str) -> String {
    id.replace('-', "")
}

fn assert_id_eq(actual: &Value, expected: &str) {
    let actual = actual.as_str().expect("actual ID should be a JSON string");

    assert_eq!(compact_id(actual), compact_id(expected));
}

fn json_id_compact(value: &Value) -> Option<String> {
    value.as_str().map(compact_id)
}

fn notion_env_available() -> bool {
    std::env::var("NOTION_API_KEY").is_ok()
}

fn require_notion_env() {
    if !notion_env_available() {
        panic!("NOTION_API_KEY is not set. Run with NOTION_API_KEY available.");
    }
}

fn swivel_cmd() -> Command {
    Command::cargo_bin("swivel").expect("failed to find swivel binary")
}

fn run_json(args: &[&str]) -> Value {
    require_notion_env();

    let output = swivel_cmd()
        .args(args)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    serde_json::from_slice(&output).expect("stdout should be valid JSON")
}

fn run_json_to_file(args: &[&str], out_path: &Path) -> Value {
    require_notion_env();

    let out_str = out_path
        .to_str()
        .expect("temp output path should be valid UTF-8");

    let mut full_args = args.to_vec();
    full_args.push("--out");
    full_args.push(out_str);

    swivel_cmd().args(&full_args).assert().success();

    let text = std::fs::read_to_string(out_path).expect("output file should exist");
    serde_json::from_str(&text).expect("output file should be valid JSON")
}

#[test]
#[ignore = "live Notion API test; run with cargo test -p swivelcli --test notion_cli_live_tests -- --ignored"]
fn get_page_returns_raw_notion_page() {
    let value = run_json(&["notion", "get-page", PAGE_IDS[0]]);

    assert_eq!(value["object"], "page");
    assert_id_eq(&value["id"], PAGE_IDS[0]);
    assert!(value["properties"].is_object());
}

#[test]
#[ignore = "live Notion API test"]
fn get_page_doc_returns_rag_document() {
    let value = run_json(&["notion", "get-page-doc", PAGE_IDS[0]]);

    assert_eq!(value["source"], "notion");
    assert_eq!(value["source_kind"], "page");
    assert_id_eq(&value["id"], PAGE_IDS[0]);

    assert!(value["title"].is_string());
    assert!(value["metadata"]["properties"].is_object());
    assert!(value["blocks"].is_array());
    assert!(value["plain_text"].is_string());
}

#[test]
#[ignore = "live Notion API test"]
fn get_page_chunks_returns_chunks_for_page() {
    let value = run_json(&["notion", "get-page-chunks", PAGE_IDS[0]]);

    let chunks = value.as_array().expect("chunks output should be an array");
    assert!(!chunks.is_empty(), "expected at least one chunk");

    for chunk in chunks {
        assert_eq!(chunk["source"], "notion");
        assert_eq!(chunk["source_kind"], "page");
        assert_id_eq(&chunk["document_id"], PAGE_IDS[0]);

        assert!(chunk["chunk_id"].is_string());
        assert!(chunk["page_title"].is_string());
        assert!(chunk["chunk_kind"].is_string());
        assert!(chunk["text"].is_string());
    }
}

#[test]
#[ignore = "live Notion API test"]
fn get_data_source_returns_raw_data_source() {
    let value = run_json(&["notion", "get-data-source", DATA_SOURCE_ID]);

    assert_id_eq(&value["id"], DATA_SOURCE_ID);
    assert!(value.is_object());
}

#[test]
#[ignore = "live Notion API test"]
fn get_data_source_docs_returns_two_page_documents() {
    let value = run_json(&["notion", "get-data-source-docs", DATA_SOURCE_ID]);

    let docs = value.as_array().expect("docs output should be an array");
    assert_eq!(
        docs.len(),
        2,
        "test data source should contain exactly two pages"
    );

    let ids = docs
        .iter()
        .filter_map(|doc| json_id_compact(&doc["id"]))
        .collect::<Vec<_>>();

    for expected_id in PAGE_IDS {
        assert!(
            ids.contains(&compact_id(expected_id)),
            "expected data source docs to include page id {expected_id}"
        );
    }

    for doc in docs {
        assert_eq!(doc["source"], "notion");
        assert_eq!(doc["source_kind"], "page");
        assert!(doc["title"].is_string());
        assert!(doc["metadata"]["properties"].is_object());
    }
}

#[test]
#[ignore = "live Notion API test"]
fn get_data_source_chunks_returns_page_chunks() {
    let value = run_json(&["notion", "get-data-source-chunks", DATA_SOURCE_ID]);

    let chunks = value.as_array().expect("chunks output should be an array");
    assert!(!chunks.is_empty(), "expected at least one chunk");

    let document_ids = chunks
        .iter()
        .filter_map(|chunk| json_id_compact(&chunk["document_id"]))
        .collect::<std::collections::BTreeSet<_>>();

    for expected_id in PAGE_IDS {
        assert!(
            document_ids.contains(&compact_id(expected_id)),
            "expected chunks for page id {expected_id}"
        );
    }
}

#[test]
#[ignore = "live Notion API test"]
fn get_database_returns_raw_database() {
    let value = run_json(&["notion", "get-database", DATABASE_ID]);

    assert_eq!(value["object"], "database");
    assert_id_eq(&value["id"], DATABASE_ID);

    assert!(
        value.is_object(),
        "database response should be a JSON object"
    );

    assert!(
        value["data_sources"].is_array(),
        "database response should include data_sources in the current Notion API model"
    );
}

#[test]
#[ignore = "live Notion API test"]
fn get_database_doc_returns_database_rag_document() {
    let value = run_json(&["notion", "get-database-doc", DATABASE_ID]);

    assert_eq!(value["source"], "notion");
    assert_eq!(value["source_kind"], "database");
    assert_id_eq(&value["id"], DATABASE_ID);

    assert!(value["title"].is_string());
    assert!(value["metadata"]["properties"].is_object());
}

#[test]
#[ignore = "live Notion API test"]
fn get_database_docs_returns_database_doc_plus_two_pages() {
    let value = run_json(&["notion", "get-database-docs", DATABASE_ID]);

    let docs = value.as_array().expect("docs output should be an array");

    assert_eq!(
        docs.len(),
        3,
        "database docs should include 1 database document + 2 page documents"
    );

    let database_docs = docs
        .iter()
        .filter(|doc| doc["source_kind"] == "database")
        .count();

    let page_docs = docs
        .iter()
        .filter(|doc| doc["source_kind"] == "page")
        .count();

    assert_eq!(database_docs, 1);
    assert_eq!(page_docs, 2);
}

#[test]
#[ignore = "live Notion API test"]
fn get_database_chunks_returns_database_summary_plus_page_chunks() {
    let value = run_json(&["notion", "get-database-chunks", DATABASE_ID]);

    let chunks = value.as_array().expect("chunks output should be an array");
    assert!(!chunks.is_empty(), "expected chunks");

    let has_database_summary = chunks.iter().any(|chunk| {
        chunk["source_kind"] == "database" && chunk["chunk_kind"] == "database_summary"
    });

    assert!(
        has_database_summary,
        "expected at least one database_summary chunk"
    );

    let has_page_chunk = chunks.iter().any(|chunk| chunk["source_kind"] == "page");

    assert!(has_page_chunk, "expected at least one page chunk");
}

#[test]
#[ignore = "live Notion API test"]
fn out_flag_writes_valid_json_file() {
    let dir = tempdir().expect("failed to create temp dir");
    let out_path = dir.path().join("page_doc.json");

    let value = run_json_to_file(&["notion", "get-page-doc", PAGE_IDS[0]], &out_path);

    assert_eq!(value["source"], "notion");
    assert_eq!(value["source_kind"], "page");
    assert_id_eq(&value["id"], PAGE_IDS[0]);
}

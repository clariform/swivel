use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use swivelcore::chunk::chunk_document;
use swivelcore::write_json_pretty;
use swivelnotion::client::NotionClient;
use swivelnotion::normalize::{
    database_to_rag_document, page_and_blocks_to_rag_document, page_to_rag_document,
};
use swiveltypes::{RagChunk, RagDocument};

#[derive(Debug, Parser)]
#[command(name = "swivel")]
#[command(about = "Source connector CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Notion {
        #[command(subcommand)]
        command: NotionCommands,
    },
}

#[derive(Debug, Subcommand)]
enum NotionCommands {
    GetPage {
        id: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    GetPageDoc {
        id: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    GetPageChunks {
        id: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    GetDataSourceDocs {
        id: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    GetDataSourceChunks {
        id: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    GetDatabaseDocs {
        id: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    GetDatabaseDoc {
        id: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    GetDatabaseChunks {
        id: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    GetDatabase {
        id: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    GetDataSource {
        id: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

fn emit_json<T: serde::Serialize>(value: &T, out: Option<PathBuf>) -> Result<()> {
    if let Some(path) = out {
        write_json_pretty(path, value)?;
    } else {
        println!("{}", serde_json::to_string_pretty(value)?);
    }
    Ok(())
}

fn build_document(client: &NotionClient, id: &str) -> Result<RagDocument> {
    let page = client.get_page_typed(id)?;
    let blocks = client.get_all_blocks_recursive(id)?;
    let doc = if blocks.is_empty() {
        page_to_rag_document(&page)
    } else {
        page_and_blocks_to_rag_document(&page, &blocks)
    };
    Ok(doc)
}

fn build_documents_for_data_source(client: &NotionClient, id: &str) -> Result<Vec<RagDocument>> {
    let pages = client.get_all_pages_for_data_source(id)?;
    let mut docs = Vec::with_capacity(pages.len());

    for page in pages {
        docs.push(build_document(client, &page.id)?);
    }

    Ok(docs)
}

fn build_database_document(client: &NotionClient, id: &str) -> Result<RagDocument> {
    let database = client.get_database_typed(id)?;
    Ok(database_to_rag_document(&database))
}

fn build_documents_for_database(client: &NotionClient, id: &str) -> Result<Vec<RagDocument>> {
    let database = client.get_database_typed(id)?;
    let mut docs: Vec<RagDocument> = Vec::new();

    docs.push(build_database_document(client, id)?);

    for data_source in database.data_sources {
        docs.extend(build_documents_for_data_source(client, &data_source.id)?);
    }

    Ok(docs)
}

fn build_chunks_for_database(client: &NotionClient, id: &str) -> Result<Vec<RagChunk>> {
    let docs = build_documents_for_database(client, id)?;
    let mut chunks = Vec::new();

    for doc in docs {
        chunks.extend(chunk_document(&doc));
    }

    Ok(chunks)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = NotionClient::from_env()?;

    match cli.command {
        Commands::Notion { command } => match command {
            NotionCommands::GetPage { id, out } => {
                let value = client.get_page_raw(&id)?;
                emit_json(&value, out)?;
            }
            NotionCommands::GetPageDoc { id, out } => {
                let doc = build_document(&client, &id)?;
                emit_json(&doc, out)?;
            }
            NotionCommands::GetPageChunks { id, out } => {
                let doc = build_document(&client, &id)?;
                let chunks = chunk_document(&doc);
                emit_json(&chunks, out)?;
            }
            NotionCommands::GetDataSourceDocs { id, out } => {
                let docs = build_documents_for_data_source(&client, &id)?;
                emit_json(&docs, out)?;
            }

            NotionCommands::GetDatabaseDoc { id, out } => {
                let doc = build_database_document(&client, &id)?;
                emit_json(&doc, out)?;
            }

            NotionCommands::GetDataSourceChunks { id, out } => {
                let docs = build_documents_for_data_source(&client, &id)?;
                let mut chunks: Vec<RagChunk> = Vec::new();

                for doc in docs {
                    chunks.extend(chunk_document(&doc));
                }

                emit_json(&chunks, out)?;
            }
            NotionCommands::GetDatabaseDocs { id, out } => {
                let docs = build_documents_for_database(&client, &id)?;
                emit_json(&docs, out)?;
            }
            NotionCommands::GetDatabaseChunks { id, out } => {
                let chunks = build_chunks_for_database(&client, &id)?;
                emit_json(&chunks, out)?;
            }
            NotionCommands::GetDatabase { id, out } => {
                let value = client.get_database_raw(&id)?;
                emit_json(&value, out)?;
            }
            NotionCommands::GetDataSource { id, out } => {
                let value = client.get_data_source_raw(&id)?;
                emit_json(&value, out)?;
            }
        },
    }

    Ok(())
}

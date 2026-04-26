use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use swivelcore::write_json_pretty;
use swivelnotion::client::NotionClient;
use swivelnotion::normalize::{page_and_blocks_to_rag_document, page_to_rag_document};

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
                let page = client.get_page_typed(&id)?;
                let blocks = client.get_all_top_level_blocks(&id)?;
                let doc = if blocks.is_empty() {
                    page_to_rag_document(&page)
                } else {
                    page_and_blocks_to_rag_document(&page, &blocks)
                };
                emit_json(&doc, out)?;
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

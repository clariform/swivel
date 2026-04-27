# Swivel

Swivel is a Rust source-retrieval and chunking toolkit for pre-RAG pipelines.

At this stage, Swivel focuses on pulling structured content from Notion, normalizing it into stable RAG document objects, and emitting RAG-ready chunks. The next layer, such as Corbin, can consume those chunks for tokenization, embeddings, and vector database storage.

Repository:

```text
https://github.com/clariform/swivel
```

Current release:

```text
v0.1.1
```

## Current scope

Swivel currently supports:

- retrieving raw Notion pages, databases, and data sources
- converting Notion pages into normalized `RagDocument` records
- converting Notion databases into database summary documents
- recursively retrieving Notion page blocks
- converting supported Notion blocks into normalized block nodes
- chunking documents into `RagChunk` records
- exposing a Rust CLI named `swivel`
- exposing a small Python wrapper package named `swivelpy`

Workspace-level retrieval is planned but not implemented yet.

## Project layout

```text
swivel
├── crates
│   ├── swivelcli       # CLI binary crate
│   ├── swivelcore      # chunking and shared core logic
│   ├── swivelnotion    # Notion API client and normalization
│   └── swiveltypes     # shared Rust data types
├── docs
│   └── architecture
├── examples
├── python
│   └── swivelpy        # Python wrapper around the swivel CLI
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Rust crates

Swivel is split into small crates:

| crate | purpose |
|---|---|
| `swiveltypes` | shared serializable data types such as `RagDocument` and `RagChunk` |
| `swivelcore` | document chunking logic |
| `swivelnotion` | Notion API client, response types, and normalization |
| `swivelcli` | command-line interface that emits JSON |

## Install the CLI

```bash
cargo install swivelcli
```

The installed binary is named:

```bash
swivel
```

Verify:

```bash
swivel --help
```

## Required environment

For Notion commands, set:

```bash
export NOTION_API_KEY="ntn_..."
export NOTION_VERSION="2026-03-11"
```

`NOTION_API_KEY` is required.

`NOTION_VERSION` is optional if the compiled default is acceptable.

## CLI usage

### Get a raw Notion page

```bash
swivel notion get-page <page_id>
```

### Get a normalized page document

```bash
swivel notion get-page-doc <page_id>
```

### Get chunks for a page

```bash
swivel notion get-page-chunks <page_id>
```

### Get raw database metadata

```bash
swivel notion get-database <database_id>
```

### Get a normalized database document

```bash
swivel notion get-database-doc <database_id>
```

### Get documents for a database

This returns the database summary document plus documents from its data sources.

```bash
swivel notion get-database-docs <database_id>
```

### Get chunks for a database

```bash
swivel notion get-database-chunks <database_id>
```

### Get raw data source metadata

```bash
swivel notion get-data-source <data_source_id>
```

### Get documents for a data source

```bash
swivel notion get-data-source-docs <data_source_id>
```

### Get chunks for a data source

```bash
swivel notion get-data-source-chunks <data_source_id>
```

### Write JSON output to a file

All commands that emit JSON support `--out`:

```bash
swivel notion get-database-chunks <database_id> --out test-output/notion/database_chunks.json
```

## Python usage

Install the Python wrapper:

```bash
pip install swivelpy
```

Or with uv:

```bash
uv add swivelpy
```

Then:

```python
from swivelpy import (
    SwivelClient,
    SwivelConfig,
    SwivelEntryKind,
    SwivelEntryPoint,
    SwivelSource,
)

client = SwivelClient(
    SwivelConfig(
        binary="swivel",
        use_cargo=False,
    )
)

entry = SwivelEntryPoint(
    source=SwivelSource.NOTION,
    kind=SwivelEntryKind.DATABASE,
    id="your-database-id",
)

chunks = client.retrieve_chunks(entry)
print(len(chunks))
```

During local development, you can call the Rust workspace through Cargo instead:

```python
from pathlib import Path

client = SwivelClient(
    SwivelConfig(
        use_cargo=True,
        project_root=Path("/path/to/swivel"),
    )
)
```

## Development

From the repository root:

```bash
cargo fmt
cargo test
cargo build
```

Run live Notion CLI tests:

```bash
cargo test -p swivelcli --test notion_cli_live_tests -- --ignored
```

These tests require `NOTION_API_KEY`.

## Packaging

Package the Rust crates in dependency order:

```bash
cargo package -p swiveltypes
cargo package -p swivelcore
cargo package -p swivelnotion
cargo package -p swivelcli
```

Publish order:

```bash
cargo publish -p swiveltypes
cargo publish -p swivelcore
cargo publish -p swivelnotion
cargo publish -p swivelcli
```

Build the Python package:

```bash
cd python/swivelpy
uv build
```

Publish the Python package:

```bash
uv publish
```

## Current boundary with Corbin

Swivel owns:

```text
source retrieval → normalization → chunk generation
```

Corbin owns:

```text
chunks → tokenization → embeddings → vector database storage
```

This keeps Swivel focused on producing clean source-derived chunks and keeps downstream systems source-agnostic.

## Status

Swivel is early but functional. The current release is intended for testing the Notion-to-chunks path and for integration with Corbin.

# swivelpy

`swivelpy` is a small Python wrapper around the Rust `swivel` CLI.

It exists so Python projects can call Swivel without reimplementing source retrieval, Notion normalization, or chunking logic.

Repository:

```text
https://github.com/clariform/swivel
```

Current release:

```text
v0.1.1
```

## What swivelpy does

`swivelpy` shells out to the installed `swivel` executable, reads JSON from stdout, and returns Python dictionaries/lists.

It does not talk to Notion directly.

Swivel owns:

```text
Notion API → normalized RagDocument → RagChunk JSON
```

Python callers own whatever comes next:

```text
RagChunk JSON → tokenization → embeddings → vector database storage
```

## Requirements

Install the Rust CLI first:

```bash
cargo install swivelcli
```

Verify:

```bash
swivel --help
```

Set Notion credentials:

```bash
export NOTION_API_KEY="ntn_..."
export NOTION_VERSION="2026-03-11"
```

## Install

With pip:

```bash
pip install swivelpy
```

With uv:

```bash
uv add swivelpy
```

For local development from the Swivel repository:

```bash
cd /path/to/swivel/python/swivelpy
uv pip install -e .
```

## Basic usage

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

## Local Cargo mode

During development, you can call the Rust workspace directly:

```python
from pathlib import Path

from swivelpy import (
    SwivelClient,
    SwivelConfig,
    SwivelEntryKind,
    SwivelEntryPoint,
    SwivelSource,
)

client = SwivelClient(
    SwivelConfig(
        use_cargo=True,
        project_root=Path("/path/to/swivel"),
    )
)

entry = SwivelEntryPoint(
    source=SwivelSource.NOTION,
    kind=SwivelEntryKind.DATABASE,
    id="your-database-id",
)

chunks = client.retrieve_chunks(entry)
```

## Entry kinds

Supported entry kinds:

| kind | meaning |
|---|---|
| `SwivelEntryKind.PAGE` | retrieve chunks from one Notion page |
| `SwivelEntryKind.DATA_SOURCE` | retrieve chunks from one Notion data source |
| `SwivelEntryKind.DATABASE` | retrieve database summary chunks and page chunks |
| `SwivelEntryKind.WORKSPACE` | reserved for future workspace-level retrieval |

## Outputs

The main public methods are:

```python
client.retrieve_chunks(entry)
client.retrieve_docs(entry)
client.retrieve_raw(entry)
```

The most important method for RAG pipelines is:

```python
client.retrieve_chunks(entry)
```

It returns a list of dictionaries shaped like Swivel `RagChunk` records.

## Development

From the `python/swivelpy` folder:

```bash
uv build
```

Test the wheel:

```bash
cd /tmp
uv venv swivelpy-wheel-test
source swivelpy-wheel-test/bin/activate
uv pip install /path/to/swivel/python/swivelpy/dist/swivelpy-*.whl
python -c "from swivelpy import SwivelClient; print(SwivelClient)"
deactivate
```

## Status

`swivelpy` is intentionally thin. It should stay small unless Swivel later needs a richer Python-native API.

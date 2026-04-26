# swivel

A modular Rust workspace for fetching, normalizing, and exporting structured content from external sources.

## workspace crates

- `swiveltypes` — shared normalized data types
- `swivelcore` — common utilities
- `swivelnotion` — Notion client and source-specific logic
- `swivelcli` — CLI entrypoint

## quick start

```bash
cargo check
cargo run -p swivelcli -- --help
```

### Example
```
export NOTION_API_KEY='secret_xxx'

cargo run -p swivelcli -- notion get-page <page_id>
cargo run -p swivelcli -- notion get-database <database_id>
cargo run -p swivelcli -- notion get-data-source <data_source_id>
```

`chargo check`
`cargo run -p swivelcli - -help`

Then test it with
```bash

export NOTION_API_KEY='secret_xxx'

cargo run -p swivelcli -- notion get-page 275a1865-b187-807a-adea-ebaf36fb49b0
```



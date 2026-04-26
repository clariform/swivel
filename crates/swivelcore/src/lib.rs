use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn write_json_pretty<P: AsRef<Path>, T: serde::Serialize>(path: P, value: &T) -> Result<()> {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(value)?;
    fs::write(path, json)?;
    Ok(())
}

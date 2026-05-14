use std::env;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};

use crate::config::schema;
const TEMPLATE: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/template/shook.toml"));

pub(super) fn execute() -> mischief::Result<()> {
    {
        let config_file = env::current_dir()?.join("shook.toml");

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&config_file)
        {
            Ok(mut file) => {
                file.write_all(TEMPLATE)
                    .wrap_err_with(|| mischief::mischief!("Failed to write to shook.toml"))?;
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                clerk::debug!("shook.toml already exists");
            }
            Err(e) => return Err(e.into()),
        };
    }
    {
        let schema_file = env::current_dir()?.join("shook.schema.json");
        let schema = serde_json::to_string_pretty(&schema())
            .wrap_err_with(|| mischief::mischief!("Failed to serialize schema"))?;
        std::fs::write(&schema_file, schema)
            .wrap_err_with(|| mischief::mischief!("Failed to write to shook.schema.json"))?;
    }
    Ok(())
}

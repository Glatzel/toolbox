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
                file.write_all(TEMPLATE)?;
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                clerk::debug!("shook.toml already exists");
            }
            Err(e) => return Err(e.into()),
        };
    }
    {
        let schema_file = env::current_dir()?.join("shook.schema.json");
        let schema = serde_json::to_string_pretty(&schema())?;
        std::fs::write(&schema_file, schema)?;
    }
    Ok(())
}

use std::io::Cursor;

use miette::Result;
use rax::io::{AsyncIRaxReader, AsyncRaxReader};
use tokio::io::BufReader; // async API

#[tokio::main]
async fn main() -> Result<()> {
    // ---------- in‑memory text -----------------------------------------------
    let data = "delta\necho\nfoxtrot";
    let cursor = Cursor::new(data.as_bytes());
    let buf_reader = BufReader::new(cursor);
    let mut reader = AsyncRaxReader::new(buf_reader);

    // ---------- read everything ----------------------------------------------
    let mut full_text = String::new();
    while let Some(line) = reader.read_line().await? {
        full_text.push_str(&line);
    }

    print!("{full_text}");
    Ok(())
}

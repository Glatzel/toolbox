use std::io::{BufReader, Cursor};

use rax::io::{IRaxReader, RaxReader}; // your crate

fn main() -> miette::Result<()> {
    // ---------- in‑memory text ------------------------------------------------
    let data = "alpha\nbravo\ncharlie"; // <- your string literal
    let cursor = Cursor::new(data.as_bytes());
    let buf_reader = BufReader::new(cursor);
    let mut reader = RaxReader::new(buf_reader);

    // ---------- read everything ----------------------------------------------
    let mut full_text = String::new();
    while let Some(line) = reader.read_line()? {
        full_text.push_str(&line); // line still contains '\n'
    }

    print!("{full_text}");
    Ok(())
}

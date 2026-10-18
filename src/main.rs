use std::env;
use std::fs;

mod dump;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path: String = env::args().nth(1).ok_or("usage: hex_dumper <file>")?;
    let bytes: Vec<u8> = fs::read(&path)?;
    for (i, chunk) in bytes.chunks(16).enumerate() {
        print!("{}", dump::format_row(i * 16, chunk));
    }
    Ok(())
}
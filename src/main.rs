use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path: String = env::args().nth(1).ok_or("usage: hex_dumper <file>")?;
    let bytes: Vec<u8> = fs::read(&path)?;
    for (i, chunk) in bytes.chunks(16).enumerate() {
        print!("{:08x}  ", i * 16);
        for b in chunk {
            print!("{:02x} ", b);
        }
        println!();
    }
    Ok(())
}
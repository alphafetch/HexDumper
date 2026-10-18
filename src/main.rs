use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path: String = env::args().nth(1).ok_or("usage: hex_dumper <file>")?;
    let text: Vec<u8> = fs::read(&path)?;
    for i in &text {
        print!("{:02x} ", i);
    }
    println!("\n");
    Ok(())
}
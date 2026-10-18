use std::env;
use std::fs;

mod dump;

fn main() {
    let path: String = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: hex_dumper <file>");
            std::process::exit(1);
        },
    };
    let bytes: Vec<u8> = match fs::read(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("hex_dumper: cannot read `{}`: {}", &path, e);
            std::process::exit(1);
        },
    };
    for (i, chunk) in bytes.chunks(16).enumerate() {
        print!("{}", dump::format_row(i * 16, chunk));
    }
}
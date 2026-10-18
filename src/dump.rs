pub fn format_row(offset: usize, chunk: &[u8]) -> String {
    let mut s: String = String::from("");
    
    s.push_str(&format!("{:08x}  ", offset));
    for b in chunk {
        s.push_str(&format!("{:02x} ", b));
    }
    s.push_str(&format!("{}", "   ".repeat(16 - chunk.len())));
    s.push_str(" |");
    for b in chunk {
        match *b as char {
            '\x20'..='\x7e' => s.push_str(&format!("{}", *b as char)),
            _ => s.push_str("."),
        }
    }
    s.push_str("|\n");

    s
}
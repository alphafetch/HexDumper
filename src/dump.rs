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

#[cfg(test)] mod tests {
    use super::*;

    #[test]
    fn full_row() {
        let row = format_row(0, b"0123456789abcdef");
        assert_eq!(
            row,
            "00000000  30 31 32 33 34 35 36 37 38 39 61 62 63 64 65 66  |0123456789abcdef|\n"
        );
    }

    #[test]
    fn short_row() {
        let row = format_row(16, b"hi\r\nyo\r\nok");
        assert_eq!(
            row,
            "00000010  68 69 0d 0a 79 6f 0d 0a 6f 6b                    |hi..yo..ok|\n"
        )
    }
}
use std::io::{Write, Result};

pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        match &self {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '!'..='/' => false,
            _ => true,
        }
    }
}

fn main() -> Result<()> {
    let mut out = Sink;
    out.write_all(b"hello world\n")?;

    assert!(!'$'.is_emoji());

    Ok(())
}

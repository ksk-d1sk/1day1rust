use std::io::{self, Write};

trait WriteHtml {
    fn write_html(&mut self, html: &str) -> io::Result<()>;
}

impl<W: Write> WriteHtml for W {  // trait를 이용하여 내부 라이브러리 구조체를 확장
    fn write_html(&mut self, html: &str) -> io::Result<()> {
        writeln!(self, "{}", html)
    }
}

fn main() {
    println!("Hello, world!");
}

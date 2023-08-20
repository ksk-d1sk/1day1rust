use std::{
    io::Write,       // 구현하는 값은 바이트열을 기록할 수 있다.
                     // : std::fs::File, std::net::TcpStream, Vec 등이 해당 trait를 구현한다.
    iter::Iterator,  // 구현하는 값은 일련의 값을 산출할 수 있다.
                     // : Split, Range 등이 해당 trait를 구현한다.
    clone::Clone,    // 구현하는 값은 메모리에 자기 자신을 복제할 수 있다.
                     // : 대부분의 표준 라이브러리 타입이 해당 trait를 구현한다.
    fmt::Debug,      // 구현하는 값은 format 처리에 {:?} 형식 지정자를 사용할 수 있다
                     // : 대부분의 표준 라이브러리 타입이 해당 trait를 구현한다.
};

fn main() -> std::io::Result<()> {
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"hello")?;              // use std::io::Write;를 선언해 주어야 사용이 가능하다.

//  let write: dyn Write = buf;            // error! 변수의 크기를 알 수 없다
    let write: &mut dyn Write = &mut buf;  // Ok!

    Ok(())
}

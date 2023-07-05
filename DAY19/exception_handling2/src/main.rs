use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = File::open("test.txt").unwrap();
    let mut f = BufReader::new(f);

    match read_numbers(&mut f) {
        Ok(v) => println!("{:?}", v),
        Err(err) => {
            println!("파일 변환중 오류 발생 {}", err.to_string());
        },
    }
}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}

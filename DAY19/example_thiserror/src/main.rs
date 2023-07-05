use thiserror::Error;

#[derive(Error, Debug)]
#[error("{message:} ({line:}, {column})")]
pub struct JsonError {
    message: String,
    line: usize,
    column: usize,
}

fn main() {
    println!("Hello, world!");
}

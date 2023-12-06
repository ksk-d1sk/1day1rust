use std::fs::OpenOptions;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut log = OpenOptions::new()
        .append(true)
        .open("happy.log")?;
    writeln!(log, "히이잉ㅠㅠㅠ 히이이잉ㅠㅠㅠㅠㅠ 히잉잉ㅠㅠㅠㅠ 힝힝힝ㅠㅠㅠㅠㅠㅠ")?;

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("new_file.txt");

    if let Some(mut new_file) = file.ok() {
        writeln!(new_file, "아와와왕")?;
    } else {
        println!("맞따끄.. 그 파일은 이미 존재한다구!");
    }

    Ok(())
}

use std::error::Error;
use std::io::{Write, stderr};

use std::fs;
use std::io;
use std::path::Path;

fn main() {
    println!("{}", pirate_share(8, 2));

    let ok: Result<i32, &str> = Ok(32);
    let err: Result<i32, &str> = Err("Error");

    assert!(ok.is_ok());
    assert!(!ok.is_err());
    assert_eq!(ok.ok(), Some(32));
    assert_eq!(ok.err(), None);
    assert_eq!(ok.unwrap_or(64), 32);
    assert_eq!(err.unwrap_or(64), 64);
    assert_eq!(err.unwrap_or_else(|err| err.len() as i32), 5);
    assert_eq!(ok.unwrap(), 32);
    assert_eq!(ok.expect("Error message"), 32);

    print_error(&"NaN".parse::<u32>().unwrap_err());

    if let Err(err) = move_all(&Path::new("foo"), &Path::new("bar")) {
        println!("IO exception {}", err.to_string());
    }
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}

fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?;
    }
    Ok(())
}

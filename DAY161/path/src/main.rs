use std::ffi::OsStr;
use std::path::Path;

fn main() {
    let home_dir = Path::new("/home/fwolfe");

    assert_eq!(home_dir.parent(), Some(Path::new("/home")));

    assert_eq!(Path::new("/home/fwolfe/program.txt").file_name(), Some(OsStr::new("program.txt")));

    let absolute_path = Path::new("C:\\hello\\world\\example.txt");
    let relative_path = Path::new("src/main.rs");

    assert!(absolute_path.is_absolute());
    assert!(!absolute_path.is_relative());

    assert!(relative_path.is_relative());
    assert!(!relative_path.is_absolute());

    let path1 = Path::new("\\usr\\share\\dict");
    assert_eq!(path1.join("words"),
               Path::new("\\usr\\share\\dict\\words"));
}

use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let path1 = Path::new("\\\\venice\\Music\\A Love Superme\\04-Psalm.mp3");
    for component in path1.components() {
        println!("{:?}", component);
    }

    let file = Path::new("/home/jimb/calendars/calendar-18x18.pdf");
    assert_eq!(file.ancestors().collect::<Vec<_>>(),
               vec![Path::new("/home/jimb/calendars/calendar-18x18.pdf"),
                    Path::new("/home/jimb/calendars"),
                    Path::new("/home/jimb"),
                    Path::new("/home"),
                    Path::new("/")]);

    if let Some(file_str) = path1.to_str() {
        println!("{}", file_str);
    }
    println!("{}", path1.to_string_lossy());  // 유효하지 않은 UTF-8 바이트 시퀀스는 전부 유니코드 대체 문자 U+FFFD로 바꾼다.
    println!("{}", file.display());           // 반환 값은 Display를 구현하고 있으므로 println!() 또는 format!() 등과 함께 사용이 가능하다.

    println!("\n-----------------\n");

    for entry_result in Path::new(".").read_dir()? {
        let entry = entry_result?;
        println!("{}", entry.path().to_string_lossy());
    }

    Ok(())
}

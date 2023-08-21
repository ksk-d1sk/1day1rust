fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let input = buf.trim();

    print!(
        "{}",
        match input {
            "M" => "MatKor",
            "W" => "WiCys",
            "C" => "CyKor",
            "A" => "AlKor",
            "$" => "$clear",
            _ => unreachable!(),
        }
    );
}
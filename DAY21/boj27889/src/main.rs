fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    print!("{}",
        match input.trim() {
            "SONGDO" => "HIGHSCHOOL",
            "CODE" => "MASTER",
            "2023" => "0611",
            "ALGORITHM" => "CONTEST",
            _ => "Bruh",
        }
    )
}

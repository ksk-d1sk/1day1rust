fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    print!("{}",
        match input.trim() {
            "NLCS" => "North London Collegiate School",
            "BHA" => "Branksome Hall Asia",
            "KIS" => "Korea International School",
            "SJA" => "St. Johnsbury Academy",
            _ => "Bruh",
        }
    )
}

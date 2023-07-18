fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    print!(
        "{}",
        if input.split_ascii_whitespace().any(|e| e == "9") {
            'F'
        } else {
            'S'
        }
    );
}

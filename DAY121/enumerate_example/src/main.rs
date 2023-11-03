fn main() {
    print_spelling("banana");
    print_spelling("Apple");
    print_spelling("Kimchi");
}

fn print_spelling(eng: &str) {
    for (i, c) in eng.chars().enumerate() {
        println!("{}번째 글자: {}", i + 1, c);
    }
    println!();
}

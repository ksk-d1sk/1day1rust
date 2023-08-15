fn main() {
    let ch = '\0';

    match ch {
        '0'..='9'             => println!("ch is number"),
        'a'..='z' | 'A'..='Z' => println!("ch is Alphabet"),
        ' ' | '\t' | '\n'     => println!("ch is whitespace"),
        _                     => println!("¯\\_(ツ)_/¯"),
    }
}

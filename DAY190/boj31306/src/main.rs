use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let mut count_vowels = 0;
    let mut count_y = 0;

    for c in input.bytes() {
        match c {
            b'a' | b'e' | b'i' | b'o' | b'u' => count_vowels += 1,
            b'y' => count_y += 1,
            _ => {},
        }
    }

    print!("{} {}", count_vowels, count_vowels + count_y);
}

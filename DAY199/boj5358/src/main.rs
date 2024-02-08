use std::io::*;

fn main() {
    print!(
        "{}",
        read_to_string(stdin())
            .unwrap()
            .chars()
            .map(|c| match c {
                'e' => 'i',
                'E' => 'I',
                'i' => 'e',
                'I' => 'E',
                _ => c,
            })
            .collect::<String>()
    );
}

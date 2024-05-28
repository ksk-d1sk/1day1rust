use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();
    let mut v: Vec<_> = input.split_ascii_whitespace().map(|s| s.to_string()).collect();

    v.sort_unstable_by(|a, b| {
        format!("{b}{a}").cmp(&format!("{a}{b}"))
    });

    for s in v {
        output.push_str(&s);
    }

    print!("{output}");
}
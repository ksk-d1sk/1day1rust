use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.lines();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    for line in (0..n).map(|_| next!()) {
        let mut chars = line.chars();
        output.push(chars.next().unwrap().to_ascii_uppercase());
        output.push_str(&chars.collect::<String>());
        output.push('\n');
    }

    print!("{output}");
}
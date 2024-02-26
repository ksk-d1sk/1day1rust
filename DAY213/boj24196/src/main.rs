use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let s = next!().as_bytes();
    let mut i = 0;

    while i < s.len() {
        output.push(s[i] as char);
        i += s[i] as usize - 64;
    }

    print!("{output}");
}
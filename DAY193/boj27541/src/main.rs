use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    next!();
    let mut s: Vec<_> = next!().chars().collect();

    if s.last() == Some(&'G') {
        s.pop();
    } else {
        s.push('G');
    }

    for c in s {
        output.push(c);
    }

    print!("{output}");
}
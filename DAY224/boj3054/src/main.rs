fn main() {
    println!("Hello, world!");
}
ause std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let s = next!().as_bytes();
    let l = s.len();

    output.push('.');

    for i in 1..=l {
        if i % 3 == 0 {
            output.push_str(".*..");
        } else {
            output.push_str(".#..");
        }
    }

    output.push_str("\n.");

    for i in 1..=l {
        if i % 3 == 0 {
            output.push_str("*.*.");
        } else {
            output.push_str("#.#.");
        }
    }

    output.push_str("\n#");

    for i in 1..=l {
        let _ = if i % 3 == 0 || (i != l && i % 3 == 2) {
            write!(output, ".{}.*", s[i - 1] as char)
        } else {
            write!(output, ".{}.#", s[i - 1] as char)
        };
    }

    output.push_str("\n.");

    for i in 1..=l {
        if i % 3 == 0 {
            output.push_str("*.*.");
        } else {
            output.push_str("#.#.");
        }
    }

    output.push_str("\n.");

    for i in 1..=l {
        if i % 3 == 0 {
            output.push_str(".*..");
        } else {
            output.push_str(".#..");
        }
    }

    print!("{output}");
}
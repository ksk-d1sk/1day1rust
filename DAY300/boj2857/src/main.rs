use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    for (i, name) in (1..=5).map(|i| (i, next!())) {
        if name.contains("FBI") {
            let _ = write!(output, "{i} ");
        }
    }

    if output.is_empty() {
        print!("HE GOT AWAY!");
    } else {
        print!("{output}");
    }
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u32);
    let p = next!(u32);

    let a = if n >= 15 {
        p.saturating_sub(2000)
    } else if n >= 5 {
        p.saturating_sub(500)
    } else {
        p
    };

    let b = if n >= 20 {
        p * 3 / 4
    } else if n >= 10 {
        p * 9 / 10
    } else {
        p
    };

    print!("{}", a.min(b));
}
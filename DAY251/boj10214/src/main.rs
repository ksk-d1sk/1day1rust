use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(usize);
    for _ in 0..t {
        let mut yt = 0;
        let mut kt = 0;

        for (y, k) in (0..9).map(|_| next!(u8, u8)) {
            yt += y;
            kt += k;
        }

        match yt.cmp(&kt) {
            Greater => output.push_str("Yonsei\n"),
            Equal   => output.push_str("Drwa\n"),
            Less    => output.push_str("Korea\n"),
        }
    }

    print!("{output}");
}
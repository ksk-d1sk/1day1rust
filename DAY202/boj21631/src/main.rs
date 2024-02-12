use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    // let input = "0 3";
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (w, b) = next!(u64, u64);

    print!("{}", b.min(w + 1));
}
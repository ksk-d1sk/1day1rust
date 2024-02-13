use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut n = next!(u64);
    let mut count = 0;

    while n != 1 {
        count += 1;
        if n & 1 == 1 {
            n += 1;
        } else {
            n /= 2;
        }
    }

    print!("{count}");
}
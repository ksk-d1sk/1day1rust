use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut n = next!(u32);
    let mut cnt = 0;

    for _ in 0..31 {
        if n & 1 == 1 {
            cnt += 1;
        }
        n >>= 1;
    }

    if cnt == 1 {
        print!("1");
    } else {
        print!("0");
    }
}
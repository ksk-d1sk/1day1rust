use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, mut k) = next!(u64, u64);
    let mut h = n / 10;
    let mut b = n % 10;

    while k > 0 {
        if b == 0 {
            k = 0;
        } else if b == 2 {
            h += k / 4 * 2;
            match k % 4 {
                0 => {
                    b = 2;
                }
                1 => {
                    b = 4;
                }
                2 => {
                    b = 8;
                }
                3 => {
                    h += 1;
                    b = 6;
                }
                _ => unreachable!(),
            }
            k = 0;
        } else {
            h += b * 2 / 10;
            b = b * 2 % 10;
            k -= 1;
        }
    }

    if h == 0 {
        print!("{b}");
    } else {
        print!("{h}{b}");
    }
}
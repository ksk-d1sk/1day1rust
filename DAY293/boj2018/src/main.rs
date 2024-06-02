use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(i32);
    let mut count = 0;
    let mut i = 1;
    let mut j = 0;
    let mut x = i;

    while j <= n {
        match n.cmp(&x) {
            Greater => {
                i += 1;
                x += i;
            }
            Equal => {
                count += 1;
                i += 1;
                x += i;
            }
            Less => {
                j += 1;
                x -= j;
            }
        }
    }

    print!("{count}");
}
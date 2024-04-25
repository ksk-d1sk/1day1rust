use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u32);


    if n & 1 == 0 {
        print!("{}", n / 10 * 2 + n % 10 / 2);
    } else {
        if n / 5 == 0 {
            print!("-1");
        } else {
            let temp = if (n / 5) & 1 == 1 {
                n / 5
            } else {
                n / 5 - 1
            };

            print!("{}", temp + (n - temp * 5) / 2);
        }
    }
}
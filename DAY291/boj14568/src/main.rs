use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u16);
    let mut count = 0;

    for i in 1..=n {
        for j in (i + 2)..=n {
            let k = i + j + 2;
            if k <= n && (n - k) & 1 == 0 {
                count += 1;
            }
        }
    }

    print!("{count}");
}
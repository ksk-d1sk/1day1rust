use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u32);
    let mut len = 0;
    let mut max = 0;
    let mut last = u32::MAX;

    for _ in 0..n {
        let q = next!();
        if q == "1" {
            let a = next!(u32);
            len += 1;

            if len > max {
                max = len;
                last = a;
            } else if len == max && a < last {
                last = a;
            }
        } else {
            len -= 1;
        }
    }

    print!("{max} {last}");
}
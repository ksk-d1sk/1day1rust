use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (t1, m1) = next!(i16, i16);
    let (t2, m2) = next!(i16, i16);

    let mut time1 = t1 * 60 + m1;
    let time2 = t2 * 60 + m2;

    if time1 > time2 {
        time1 -= 24 * 60;
    }

    let res = time2 - time1;

    print!("{} {}", res, res / 30);
}
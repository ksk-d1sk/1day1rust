use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (e, s, m) = next!(u32, u32, u32);

    for i in 1..=(15 * 28 * 19) {
        let d1 = (i + 14) % 15 + 1;
        let d2 = (i + 27) % 28 + 1;
        let d3 = (i + 18) % 19 + 1;
        if d1 == e && d2 == s && d3 == m {
            print!("{i}");
            break;
        }
    }
}
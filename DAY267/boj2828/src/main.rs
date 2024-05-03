use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (_, m, j) = next!(u8, u8, u8);
    let mut l = 1;
    let mut r = m;
    let mut answer = 0;

    for a in (0..j).map(|_| next!(u8)) {
        if a < l {
            let mv = l - a;
            answer += mv;
            l -= mv;
            r -= mv;
        } else if r < a {
            let mv = a - r;
            answer += mv;
            l += mv;
            r += mv;
        }
    }

    print!("{answer}");
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, w, h) = next!(u8, u32, u32);
    let ww = w * w;
    let hh = h * h;

    for x in (0..n).map(|_| next!(u32)) {
        if x * x > ww + hh {
            output.push_str("NE\n");
        } else {
            output.push_str("DA\n");
        }
    }

    print!("{output}");
}
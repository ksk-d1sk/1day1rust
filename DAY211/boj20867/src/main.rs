use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let (m, s, g) = next!(u32, u32, u32);
    let (a, b) = next!(f64, f64);
    let (l, r) = next!(f64, f64);

    if (l / a) as u32 + (m + g - 1) / g < (r / b) as u32 + (m + s - 1) / s {
        print!("friskus");
    } else {
        print!("latmask");
    }
}
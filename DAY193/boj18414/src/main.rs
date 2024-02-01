use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (x, l ,r) = next!(u32, u32, u32);

    if x < l {
        print!("{l}");
    } else if x > r {
        print!("{r}");
    } else {
        print!("{x}");
    }
}
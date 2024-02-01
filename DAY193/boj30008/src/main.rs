use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = next!(u32, u8);
    for d in (0..k).map(|_| next!(u32)) {
        print!(
            "{} ",
            match d * 100 / n {
                97.. => 9,
                90.. => 8,
                78.. => 7,
                61.. => 6,
                41.. => 5,
                24.. => 4,
                12.. => 3,
                5.. => 2,
                0.. => 1,
            }
        );
    }
}
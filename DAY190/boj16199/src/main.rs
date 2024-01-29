use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (ay, am, ad) = next!(u16, u8, u8);
    let (by, bm, bd) = next!(u16, u8, u8);

    print!(
        "{}\n{}\n{}",
        by - ay - !(am < bm || (am == bm && ad <= bd)) as u16,
        by - ay + 1,
        by - ay,
    );
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (ca, ba, pa) = next!(u16, u16, u16);
    let (cr, br, pr) = next!(u16, u16, u16);

    print!("{}", cr.saturating_sub(ca) + br.saturating_sub(ba) + pr.saturating_sub(pa));
}
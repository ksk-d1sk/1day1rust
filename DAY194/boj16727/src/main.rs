use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (p1, s1) = next!(u8, u8);
    let (s2, p2) = next!(u8, u8);

    print!(
        "{}",
        match (p1 + p2).cmp(&(s1 + s2)) {
            Greater => "Persepolis",
            Equal => match p2.cmp(&s1) {
                Greater => "Persepolis",
                Equal => "Penalty",
                Less => "Esteghlal",
            },
            Less => "Esteghlal",
        }
    );
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let m = next!(u8);
    let mut v = [false, true, false, false];

    for (x, y) in (0..m).map(|_| next!(usize, usize)) {
        (v[x], v[y]) = (v[y], v[x]);
    }

    print!("{}", v.iter().zip(0..).filter_map(|(b, i)| b.then_some(i)).next().unwrap());
}
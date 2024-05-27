use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut v = Vec::from_iter((0..n).map(|_| next!(u32)));
    let mut h_index = 0;

    v.sort_unstable_by(|a, b| b.cmp(a));

    for (x, i) in v.iter().zip(1..) {
        if *x >= i {
            h_index += 1;
        } else {
            break;
        }
    }

    print!("{h_index}");
}
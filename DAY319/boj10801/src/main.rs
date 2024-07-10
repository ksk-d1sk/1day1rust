use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let a = Vec::from_iter((0..10).map(|_| next!(u8)));
    let b = Vec::from_iter((0..10).map(|_| next!(u8)));
    let mut a_count = 0;
    let mut b_coubt = 0;

    for i in 0..10 {
        if a[i] > b[i] {
            a_count += 1;
        } else if a[i] < b[i] {
            b_coubt += 1;
        }
    }

    match a_count.cmp(&b_coubt) {
        Greater => print!("A"),
        Equal => print!("D"),
        Less => print!("B"),
    }
}
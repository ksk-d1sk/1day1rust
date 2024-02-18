use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let k = next!(u8);
    let mut count_a = 0;
    let mut count_b = 1;

    for _ in 1..k {
        (count_a, count_b) = (count_b, count_a + count_b);
    }

    print!("{count_a} {count_b}");
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut answer = 0;
    let mut min = u32::MAX;

    for x in (0..n).map(|_| next!(u32)) {
        if x > min {
            answer = answer.max(x - min);
        } else if x < min {
            min = x;
        }
    }

    print!("{answer}");
}
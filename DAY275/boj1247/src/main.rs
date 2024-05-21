use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    for _ in 0..3 {
        let n = next!(u32);
        let sum: i128 = (0..n).map(|_| next!(i128)).sum();
        output.push(
            match sum.cmp(&0) {
                Greater => '+',
                Equal => '0',
                Less => '-',
            }
        );
        output.push('\n');
    }

    print!("{output}");
}
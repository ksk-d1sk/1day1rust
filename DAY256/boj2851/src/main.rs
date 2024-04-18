use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = 0;
    let mut sum = 0_i16;
    let mut min_diff = u16::MAX;

    for x in (0..10).map(|_| next!(i16)){
        sum += x;
        let diff = sum.abs_diff(100);
        if min_diff >= diff {
            answer = sum;
            min_diff = diff;
        }
    }

    print!("{answer}");
}
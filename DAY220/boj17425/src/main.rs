use std::fmt::Write;
use std::io::*;

const MAX: usize = 1_000_000;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut prefix_sum_of_div = vec![1; MAX + 1];
    for i in 2..=MAX {
        for j in (i..=MAX).step_by(i) {
            prefix_sum_of_div[j] += i;
        }
        prefix_sum_of_div[i] += prefix_sum_of_div[i - 1];
    }

    let t = next!(usize);
    for n in (0..t).map(|_| next!(usize)) {
        let _ = writeln!(output, "{}", prefix_sum_of_div[n]);
    }

    print!("{output}");
}
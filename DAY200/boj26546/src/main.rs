use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(usize);
    for _ in 0..t {
        let s = next!();
        let (i, j) = next!(usize, usize);

        for (k, c) in s.chars().enumerate() {
            if k < i || k >= j {
                output.push(c);
            }
        }

        output.push('\n');
    }

    print!("{output}");
}
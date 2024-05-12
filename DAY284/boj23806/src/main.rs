use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let m = n * 5;
    for i in 0..m {
        if i >= n && i < n * 4 {
            for _ in 0..n {
                output.push('@');
            }

            for _ in 0..(n * 3) {
                output.push(' ');
            }

            for _ in 0..n {
                output.push('@');
            }
        } else {
            for _ in 0..m {
                output.push('@');
            }
        }

        output.push('\n');
    }

    print!("{output}");
}
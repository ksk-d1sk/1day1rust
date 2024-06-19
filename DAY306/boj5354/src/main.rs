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
    for n in (0..t).map(|_| next!(u8)) {
        for i in 0..n {
            if i == 0 || i == n - 1 {
                for _ in 0..n {
                    output.push('#');
                }
            } else {
                output.push('#');
                for _ in 0..(n - 2) {
                    output.push('J');
                }
                output.push('#');
            }
            output.push('\n');
        }
        output.push('\n');
    }

    print!("{output}");
}
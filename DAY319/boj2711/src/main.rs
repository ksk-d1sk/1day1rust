use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(u16);
    for (i, s) in (0..t).map(|_| next!(usize, String)) {
        for (c, j) in s.chars().zip(1..) {
            if i != j {
                output.push(c);
            }
        }

        output.push('\n');
    }

    print!("{output}");
}
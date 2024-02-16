use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);
    for (i, f) in (0..n).map(|_| next!(u8, u8)) {
        let (max, min) = if i > f {
            (i, f)
        } else {
            (f, i)
        };

        if max > 2 || min > 1{
            output.push_str("No\n");
        } else {
            output.push_str("Yes\n");
        }
    }

    print!("{output}");
}
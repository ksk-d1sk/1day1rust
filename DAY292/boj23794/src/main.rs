use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);

    for _ in 0..n+2 { output.push('@'); }
    output.push('\n');
    for _ in 0..n {
        output.push('@');
        for _ in 0..n { output.push(' '); }
        output.push('@');
        output.push('\n');
    }
    for _ in 0..n+2 { output.push('@'); }
    print!("{output}");
}
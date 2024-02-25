use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u32);

    for _ in 0..n { output.push_str(" @@@   @@@  ") } output.push('\n');
    for _ in 0..n { output.push_str("@   @ @   @ ") } output.push('\n');
    for _ in 0..n { output.push_str("@    @    @ ") } output.push('\n');
    for _ in 0..n { output.push_str("@         @ ") } output.push('\n');
    for _ in 0..n { output.push_str(" @       @  ") } output.push('\n');
    for _ in 0..n { output.push_str("  @     @   ") } output.push('\n');
    for _ in 0..n { output.push_str("   @   @    ") } output.push('\n');
    for _ in 0..n { output.push_str("    @ @     ") } output.push('\n');
    for _ in 0..n { output.push_str("     @      ") }

    print!("{output}");
}
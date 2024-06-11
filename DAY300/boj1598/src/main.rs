use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b) = next!(i32, i32);
    let ax = (a - 1) / 4;
    let ay = (a - 1) % 4;
    let bx = (b - 1) / 4;
    let by = (b - 1) % 4;

    print!("{}", ax.abs_diff(bx) + ay.abs_diff(by));
}
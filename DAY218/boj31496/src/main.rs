use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u32);
    let birch = next!();
    let mut answer = 0;

    for _ in 0..n {
        let item = next!();
        let amount = next!(u32);

        if item.split('_').any(|s| s == birch) {
            answer += amount;
        }
    }

    print!("{answer}");
}
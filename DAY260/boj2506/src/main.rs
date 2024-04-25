use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut answer = 0;
    let mut i = 0;

    for ox in (0..n).map(|_| next!()) {
        if ox == "1" {
            i += 1;
            answer += i;
        } else {
            i = 0;
        }
    }

    print!("{answer}");
}
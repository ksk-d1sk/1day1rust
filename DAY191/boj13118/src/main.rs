use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = 0;
    let v = [next!(i32), next!(i32), next!(i32), next!(i32)];
    let a = next!(i32);

    for (i, x) in v.into_iter().enumerate() {
        if x == a {
            answer = i + 1;
        }
    }

    print!("{answer}");
}
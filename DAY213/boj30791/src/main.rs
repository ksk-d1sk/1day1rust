use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let v: Vec<_> = (0..5).map(|_| next!(u16)).collect();
    let mut count = 0;

    for i in 1..5 {
        if v[i] >= v[0] - 1000 {
            count += 1;
        }
    }

    print!("{count}");
}
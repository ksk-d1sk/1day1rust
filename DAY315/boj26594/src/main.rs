use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let s = next!().as_bytes();
    let t = Some(&s[0]);
    let mut answer = 0;

    for i in 1.. {
        if t != s.get(i) {
            answer = i;
            break;
        }
    }

    print!("{answer}");
}
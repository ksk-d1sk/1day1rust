use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.lines();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut s = next!().chars();
    let check = s.next() == Some('"') && s.next_back() == Some('"');
    let answer = s.collect::<String>();

    if check && !answer.is_empty() {
        print!("{answer}");
    } else {
        print!("CE");
    }
}
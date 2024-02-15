use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut count_a = 0;
    let mut count_b = 0;

    for b in next!().bytes() {
        if b == b'A' {
            count_a += 1;
        } else {
            count_b += 1;
        }
    }

    print!("{} : {}", count_a, count_b);    
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let s = next!();
    let mut sum = 0;
    let mut check = true;

    for b in s.bytes() {
        match b {
            b'a'..=b'z' => sum += (1 + b - b'a') as u32,
            b'A'..=b'Z' => sum += (27 + b - b'A') as u32,
            _ => unreachable!(),
        }
    }

    for i in (2..).take_while(|i| i * i <= sum) {
        if sum % i == 0 {
            check = false;
            break;
        }
    }

    if check {
        print!("It is a prime word.");
    } else {
        print!("It is not a prime word.");
    }
}
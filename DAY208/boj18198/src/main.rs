use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $(t:ty),+ ) => { ($(tokens.next().unwrap().parse::<t>().unwrap()),+) }
    }

    let mut iter = next!().bytes();
    let mut socre_a = 0_i32;
    let mut socre_b = 0;

    while let Some(b) = iter.next() {
        if b == b'A' {
            socre_a += (iter.next().unwrap() - b'0') as i32;
        } else {
            socre_b += (iter.next().unwrap() - b'0') as i32;
        }

        if (socre_a > 10 || socre_b > 10) && (socre_a - socre_b).abs() > 1 {
            if socre_a > socre_b {
                print!("A");
            } else {
                print!("B");
            }

            break;
        }
    }
}
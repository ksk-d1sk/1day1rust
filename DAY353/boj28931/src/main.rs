use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u32);
    let a1 = next!(u32);
    let a1m3 = a1 % 3;

    if n == 1 {
        print!("{a1m3}");
    } else {
        let a2 = next!(u32);
    
        if a1m3 == 0 {
            print!("0");
        } else if a1m3 == 1 {
            print!("1");
        } else if a2 & 1 == 1 {
            print!("2");
        } else {
            print!("1");
        }
    }
}
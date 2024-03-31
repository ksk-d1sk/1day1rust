use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let junhee_is_cute = (0..n).filter(|_| next!() == "1").count();

    if junhee_is_cute > n / 2 {
        print!("Junhee is cute!");
    } else {
        print!("Junhee is not cute!");
    }
}
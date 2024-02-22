use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split('-');

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let (_, m, d) = next!(u16, u8, u8);
    if m < 9 || (m == 9 && d <= 16) {
        print!("GOOD");
    } else {
        print!("TOO LATE");
    }
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (t1, v1) = next!(i8, i8);
    let (t2, v2) = next!(i8, i8);

    if t2 < 0 && v2 >= 10 {
        print!("A storm warning for tomorrow! Be careful and stay home if possible!");
    } else
    if t2 < t1 {
        print!("MCHS warns! Low temperature is expected tomorrow.");
    } else
    if t2 >= t1 && v2 > v1 {
        print!("MCHS warns! Strong wind is expected tomorrow.");
    } else
    {
        print!("No message");
    }
}
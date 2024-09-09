use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut x = next!(i32);
    loop {
         match next!() {
            "+" => x += next!(i32),
            "-" => x -= next!(i32),
            "*" => x *= next!(i32),
            "/" => x /= next!(i32),
             _ => break,
         }
    }

    print!("{x}");
}
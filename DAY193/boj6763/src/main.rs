use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let limit = next!(u32);
    let speed = next!(u32);

    if speed > limit {
        print!(
            "You are speeding and your fine is ${}.",
            match speed - limit {
                31.. => 500,
                21.. => 270,
                _    => 100,
            }
        )
    } else {
        print!("Congratulations, you are within the speed limit!");
    }
}
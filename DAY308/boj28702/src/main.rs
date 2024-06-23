use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $t:ty) => { tokens.next().unwrap().parse::<$t>() };
    }

    let mut answer = 0;

    for _ in 0..3 {
        if let Ok(x) = next!(u32) {
            answer = x;
        }

        answer += 1;
    }

    if answer % 3 == 0 && answer % 5 == 0 {
        print!("FizzBuzz");
    } else if answer % 3 == 0 {
        print!("Fizz");
    } else if answer % 5 == 0 {
        print!("Buzz");
    } else {
        print!("{answer}");
    }
}
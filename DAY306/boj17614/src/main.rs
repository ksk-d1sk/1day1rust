use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u32);
    let mut answer = 0_usize;

    for mut i in 3..=n {
        while i > 0 {
            let x = i % 10;
            if x != 0 && x % 3 == 0 {
                answer += 1;
            }
            i /= 10;
        }
    }

    print!("{answer}");
}
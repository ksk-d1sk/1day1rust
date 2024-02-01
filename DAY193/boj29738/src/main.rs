use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(u8);

    for i in 1..=t {
        println!(
            "Case #{i}: {}",
            match next!(u16) {
                4501.. => "Round 1",
                1001.. => "Round 2",
                26..   => "Round 3",
                _      => "World Finals",
            }
        );
    }
}
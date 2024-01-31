use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let lv = next!(u16);

    print!(
        "{}",
        match next!() {
            "miss" => 0,
            "bad" => lv * 200,
            "cool" => lv * 400,
            "great" => lv * 600,
            "perfect" => lv * 1000,
            _ => unreachable!(),
        }
    );
}
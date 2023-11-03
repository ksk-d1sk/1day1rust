use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (x, y, n) = get!(u8, u8, u8);

    for i in 1..=n {
        let _ = writeln!(
            output,
            "{}",
            if i % x == 0 && i % y == 0 {
                "FizzBuzz".to_string()
            } else if i % x == 0 {
                "Fizz".to_string()
            } else if i % y == 0 {
                "Buzz".to_string()
            } else {
                i.to_string()
            }
        );
    }

    print!("{output}");
}

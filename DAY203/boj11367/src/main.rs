use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    for (name, score) in (0..n).map(|_| (next!(), next!(u8))) {
        let _ = writeln!(
            output,
            "{name} {}",
            match score {
                97.. => "A+",
                90.. => "A",
                87.. => "B+",
                80.. => "B",
                77.. => "C+",
                70.. => "C",
                67.. => "D+",
                60.. => "D",
                _    => "F",
            }
        );
    }

    print!("{output}");
}
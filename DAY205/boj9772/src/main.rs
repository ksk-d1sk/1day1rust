use std::cmp::Ordering::*;
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

    for (x, y) in (0..).map(|_| next!(f64, f64)).take_while(|(x, y)| *x != 0.0 || *y != 0.0) {
        let _ = writeln!(
            output,
            "{}",
            match (x.partial_cmp(&0.0).unwrap(), y.partial_cmp(&0.0).unwrap()) {
                (Greater, Greater) => "Q1",
                (Less, Greater)    => "Q2",
                (Less, Less)       => "Q3",
                (Greater, Less)    => "Q4",
                _ => "AXIS",
            }
        );
    }

    print!("{output}AXIS");
}
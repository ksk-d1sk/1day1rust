use std::fmt::Write;
use std::cmp::Ordering::*;
use std::io::Read;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $( $t:ty ),+ ) => { ( $(input.next().unwrap().parse::<$t>().unwrap() ),+ ) };
    }

    let n = get!(usize);

    for _ in 0..n {
        let (a, b, c, d) = get!(u64, u64, u64, u64);
        let r1 = a * b;
        let r2 = c * d;
    
        let _ = writeln!(
            output, "{}",
            match r1.cmp(&r2) {
                Greater => "TelecomParisTech",
                Equal => "Tie",
                Less => "Eurecom",
            }
        );
    }

    print!("{output}");
}

use std::io::Read;
use std::fmt::Write;

fn main() {
    // let mut f = std::fs::File::open("input.txt").unwrap();
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // f.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $( $t:ty ),+ ) => { ( $(input.next().unwrap().parse::<$t>().unwrap() ),+ ) };
    }

    for _ in 0..get!(usize) {
        let (d, f, p) = get!(f64, f64, f64);
        let _ = writeln!(output, "${:.02}", d * f * p);
    }

    print!("{output}");
}

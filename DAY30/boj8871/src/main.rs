use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $( $t:ty ),+ ) => { ( $(input.next().unwrap().parse::<$t>().unwrap() ),+ ) };
    }

    let n = get!(u16);

    println!("{} {}", (n + 1) * 2, (n + 1) * 3);
}

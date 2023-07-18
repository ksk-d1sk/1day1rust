fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $( $t:ty ),+ ) => { ( $(input.next().unwrap().parse::<$t>().unwrap() ),+ ) };
    }

    let (a, b, c, d) = get!(u32, u32, u32, u32);

    print!("{}", a * 56 + b * 24 + c * 14 + d * 6);
}

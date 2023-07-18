use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $( $t:ty ),+ ) => { ( $(input.next().unwrap().parse::<$t>().unwrap() ),+ ) };
    }

    let (t1, f1, s1, p1, c1) = get!(u8, u8, u8, u8, u8);
    let (t2, f2, s2, p2, c2) = get!(u8, u8, u8, u8, u8);

    print!(
        "{} {}",
        t1 * 6 + f1 * 3 + s1 * 2 + p1 * 1 + c1 * 2,
        t2 * 6 + f2 * 3 + s2 * 2 + p2 * 1 + c2 * 2,
    );
}

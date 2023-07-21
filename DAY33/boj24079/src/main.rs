use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("input.txt").unwrap();
    let mut input = String::new();
    // std::io::stdin().read_to_string(&mut input).unwrap();
    f.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $( $t:ty ),+ ) => { ( $(input.next().unwrap().parse::<$t>().unwrap() ),+ ) };
    }

    let (x, y, z) = get!(u8, u8, u8);

    print!("{}", u8::from(x + y <= z));
}

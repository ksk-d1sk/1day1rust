use std::io::Read;

fn main() {
    // let mut f = std::fs::File::open("input.txt").unwrap();
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // f.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b) = get!(u16, u16);

    print!(
        "{}",
        match 218.cmp(&(a * 100 + b)) {
            std::cmp::Ordering::Greater => "Before",
            std::cmp::Ordering::Equal => "Special",
            std::cmp::Ordering::Less => "After",
        }
    )
}

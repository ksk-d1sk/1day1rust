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

    let (m, n) = get!(u32, u32);
    let mut iter = (1..=100).map(|e| e * e).filter(|&e| m <= e && e <= n);

    if let Some(min) = iter.next() {
        print!("{}\n{}", iter.sum::<u32>() + min, min);
    } else {
        print!("-1");
    }
}

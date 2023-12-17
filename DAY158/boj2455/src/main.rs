use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut max = 0;
    let mut count = 0;

    for (a, b) in (0..4).map(|_| get!(u16 ,u16)) {
        count = count + b - a;

        if count > max {
            max = count;
        }
    }

    print!("{max}");
}
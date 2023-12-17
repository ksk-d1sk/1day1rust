use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let a = get!(u8);
    let mut count = 0;

    for i in (0..5).map(|_| get!(u8)) {
        if a == i {
            count += 1;
        }
    }

    print!("{count}");
}
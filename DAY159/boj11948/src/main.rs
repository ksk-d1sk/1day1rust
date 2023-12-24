use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut arr1 = Vec::with_capacity(4);
    let mut arr2 = Vec::with_capacity(2);

    for e in (0..4).map(|_| get!(u16)) {
        arr1.push(e);
    }

    for e in (0..2).map(|_| get!(u16)) {
        arr2.push(e);
    }

    let a = arr1.iter().sum::<u16>() - arr1.iter().min().unwrap();
    let b = arr2.iter().sum::<u16>() - arr2.iter().min().unwrap();

    print!("{}", a + b);
}
use std::io;
use std::collections::HashSet;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let mut count = 0;
    let mut map: HashSet<i32> = (0..n).map(|_| get!(i32)).collect();

    (0..m).map(|_| get!(i32)).for_each(|x| {
        if !map.remove(&x) {
            count += 1;
        }
    });

    count += map.len();

    print!("{count}");
}
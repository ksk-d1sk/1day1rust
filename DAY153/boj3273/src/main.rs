use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    // let buf = include_str!("input.txt");
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut v: Vec<_> = (0..n).map(|_| get!(u32)).collect();

    v.sort_unstable();

    let x = get!(u32);
    let mut count = 0;
    let mut i = 0;
    let mut j = n - 1;

    while i < j {
        let sum = v[i] + v[j];
        if sum == x {
            count += 1;
            i += 1;
            j -= 1;
        } else if sum < x {
            i += 1;
        } else {
            j -= 1;
        }
    }

    print!("{count}");
}

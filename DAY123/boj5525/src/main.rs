use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let s = get!();
    let cmp_s = vec!["I"; n + 1].join("O");
    let l = n * 2 + 1;
    let mut count = 0;

    for i in 0..=(m - l) {
        if &cmp_s == &s[i..(i + l)] {
            count += 1;
        }
    }

    print!("{count}");
}
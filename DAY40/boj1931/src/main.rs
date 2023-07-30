use std::io;

fn main() {
    // let mut f = std::fs::File::open("input.txt").unwrap();
    let input = io::read_to_string(io::stdin()).unwrap();
    // f.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut v: Vec<_> = Vec::with_capacity(n);

    for _ in 0..n {
        v.push(get!(u32, u32));
    }

    v.sort_unstable_by_key(|&(s, e)| (e, s));

    let mut last_processed_end_time = 0;
    let mut count = 0;

    for (s, e) in v {
        if last_processed_end_time <= s {
            last_processed_end_time = e;
            count += 1;
        }
    }

    print!("{count}");
}

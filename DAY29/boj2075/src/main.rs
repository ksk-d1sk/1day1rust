use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => {{
            input.clear();
            stdin.read_line(&mut input).unwrap();
            let mut iter = input.split_ascii_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),+ )
        }};
    }

    macro_rules! get_line {
        ( $t:ty ) => {{
            input.clear();
            stdin.read_line(&mut input).unwrap();
            input.split_ascii_whitespace().flat_map(str::parse::<$t>)
        }};
    }

    let n = get!(usize);
    let mut v = Vec::with_capacity(n);

    for _ in 0..n {
        for e in get_line!(i32) {
            v.push(e);
        }
    }

    print!("{}", v.select_nth_unstable_by(n - 1, |a, b| b.cmp(a)).1);
}
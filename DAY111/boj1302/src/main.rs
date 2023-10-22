use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    print!(
        "{}",
        loop {
            let (n, m) = get!(usize, usize);
            if n == 0 && m == 0 {
                break output;
            }
            let mut set = HashSet::with_capacity(n);
            let mut count = 0;

            for _ in 0..n {
                set.insert(get!());
            }

            for _ in 0..m {
                if set.contains(get!()) {
                    count += 1;
                }
            }

            let _ = writeln!(output, "{count}");
        }
    );
}

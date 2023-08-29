use std::io;
use std::collections::HashMap;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let mut set = HashMap::with_capacity(n);

    for _ in 0..n {
        set.insert(get!(), get!());
    }

    for _ in 0..m {
        output.push_str(set.get(get!()).unwrap());
        output.push('\n');
    }

    print!("{output}");
}
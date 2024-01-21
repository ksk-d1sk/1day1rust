use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut count = 0;
    let mut stack = Vec::with_capacity(100_000);
    let n = get!(u8);

    for s in (0..n).map(|_| get!()) {
        if s.len() & 1 == 1 { continue }

        for b in s.bytes() {
            if stack.last() == Some(&b) {
                stack.pop();
            } else {
                stack.push(b);
            }
        }

        count += stack.is_empty() as u8;
        stack.clear();
    }

    print!("{count}");
}
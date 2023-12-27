use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let s = get!().as_bytes();
    let mut i = 1;
    let mut answer = 0;

    while i < m {
        if s[i - 1] == b'I' && s[i] == b'O' {
            let mut count = 0;

            while s.get(i) == Some(&b'O') && s.get(i + 1) == Some(&b'I') {
                i += 2;
                count += 1;
                if count >= n {
                    answer += 1;
                }
            }
        }

        i += 1;
    }

    print!("{answer}");
}
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, mut b) = get!(u32, u32);
    let mut answer = 1;

    while a < b {
        answer += 1;

        if b % 10 == 1 {
            b /= 10;
        } else if b & 1 == 0 {
            b /= 2;
        } else {
            answer = -1;
            break;
        }
    }

    print!("{}", if b < a { -1 } else { answer });
}
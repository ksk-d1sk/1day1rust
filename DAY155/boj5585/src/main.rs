use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut n = 1000 - get!(u16);
    let mut answer = 0;

    for i in [500, 100, 50, 10, 5, 1] {
        answer += n / i;
        n %= i;
    }

    print!("{answer}");
}

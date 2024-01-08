use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $t:ty ) => { input.next().unwrap().parse::<$t>().unwrap() };
    }

    let mut n = get!(i64);
    let mut count = 0;
    let mut check = false;

    while 0 <= n {
        if n % 5 == 0 {
            check = true;
            break;
        }

        n -= 3;
        count += 1;
    }

    if check {
        print!("{}", count + n / 5);
    } else {
        print!("-1");
    }
}
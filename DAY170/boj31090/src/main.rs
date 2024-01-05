use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = get!(u16);
    for n in (0..t).map(|_| get!(u16)) {
        let m = n % 100;

        if (n + 1) % m == 0 {
            output.push_str("Good\n");
        } else {
            output.push_str("Bye\n");
        }
    }

    print!("{output}");
}
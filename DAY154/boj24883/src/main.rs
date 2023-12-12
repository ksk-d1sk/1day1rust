use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    print!(
        "Naver {}",
        match get!(char) {
            'N' | 'n' => "D2",
            _ => "Whale",
        }
    )
}

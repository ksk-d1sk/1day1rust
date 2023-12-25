use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut v = vec![(u16::MAX, "OK")];
    let a = get!(u16);
    let b = get!(u16);
    let c = get!(u16);

    v.push((a, "Soongsil"));
    v.push((b, "Korea"));
    v.push((c, "Hanyang"));

    if a + b + c < 100 {
        v.sort();
    }

    print!("{}", v[0].1);
}

use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let p = get!(u8);
    let mut v = vec![0; 4];
    for (g, c, _) in (0..p).map(|_| get!(usize, usize, usize)) {
        if g == 1{
            v[3] += 1;
        } else {
            match c {
                3 => v[1] += 1,
                4 => v[2] += 1,
                _ => v[0] += 1,
            }
        }
    }

    for i in v {
        println!("{i}");
    }
}
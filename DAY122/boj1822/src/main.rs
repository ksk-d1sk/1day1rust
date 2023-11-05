use std::collections::BTreeSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (na, nb) = get!(usize, usize);
    let mut set: BTreeSet<_> = (0..na).map(|_| get!(i32)).collect();

    for x in (0..nb).map(|_| get!(i32)) {
        set.remove(&x);
    }

    let _ = writeln!(output, "{}", set.len());

    for x in set {
        let _ = write!(output, "{} ", x);
    }

    print!("{output}");
}
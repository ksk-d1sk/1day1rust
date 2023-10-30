use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut dancing = HashMap::with_capacity(n * 2);
    let mut meeting = Vec::with_capacity(n);

    for (val1, val2) in (0..n).map(|_| (get!(), get!())) {
        meeting.push((val1, val2));
        dancing.insert(val1, false);
        dancing.insert(val2, false);
    }

    if let Some(v) = dancing.get_mut("ChongChong") {
        *v = true;
    }

    let mut count = 1;

    for (k1, k2) in meeting {
        if dancing.get(k1).unwrap() ^ dancing.get(k2).unwrap() {
            count += 1;

            if let Some(v) = dancing.get_mut(k1) {
                *v = true;
            }

            if let Some(v) = dancing.get_mut(k2) {
                *v = true;
            }
        }
    }

    print!("{count}");
}
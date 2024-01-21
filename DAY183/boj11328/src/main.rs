use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $t:ty ) => { input.next().unwrap().parse::<$t>().unwrap() };
    }

    let n = get!(u16);

    for (a, b) in (0..n).map(|_| (get!(), get!())) {
        let mut v = [0_i16; 26];

        for i in a.bytes() {
            v[(i - b'a') as usize] += 1;
        }

        for i in b.bytes() {
            v[(i - b'a') as usize] -= 1;
        }

        if v.into_iter().filter(|x| *x != 0).count() == 0 {
            output.push_str("Possible\n");
        } else {
            output.push_str("Impossible\n");
        }
    }

    print!("{output}");
}
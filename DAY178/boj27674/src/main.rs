use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $t:ty ) => { input.next().unwrap().parse::<$t>().unwrap() };
    }

    let t = get!(u16);
    for s in (0..t).map(|_| get!()) {
        let mut v: Vec<_> = s.bytes().collect();
        
        v.sort_unstable_by(|b, a| a.cmp(b));

        
        let mut a = 0;
        let b = (v.pop().unwrap() - b'0') as u64;

        for x in v {
            a = a * 10 + (x - b'0') as u64;
        }

        let _ = writeln!(output, "{}", a + b);
    }

    print!("{output}");
}
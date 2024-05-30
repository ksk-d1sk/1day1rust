use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.lines();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut v1 = Vec::from_iter((0..8).map(|_| next!(u32)).zip(1_u8..));
    let mut v2 = Vec::new();
    let mut sum = 0;

    v1.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    for (x, i) in &v1[0..5] {
        sum += x;
        v2.push(i);
    }

    v2.sort_unstable();

    let _ = writeln!(output, "{sum}");

    for i in v2 {
        let _ = write!(output, "{i} ");
    }

    print!("{output}");
}
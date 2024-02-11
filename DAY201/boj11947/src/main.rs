use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(usize);
    for n in (0..t).map(|_| next!(u64)) {
        let mut temp = n;
        let mut a = 5;

        while temp > 9 {
            a *= 10;
            temp /= 10;
        }

        let _ = writeln!(output, "{}", lovely(n.min(a)));
    }

    print!("{output}");
}

fn lovely(n: u64) -> u64 {
    n * rev(n)
}

fn rev(n: u64) -> u64 {
    let mut ret = 9;
    let mut temp = n;

    while temp > 9 {
        ret = ret * 10 + 9;
        temp /= 10;
    }

    ret - n
}
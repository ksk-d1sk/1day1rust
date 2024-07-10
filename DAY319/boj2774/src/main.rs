use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(usize);
    for mut x in (0..t).map(|_| next!(usize)) {
        let mut arr = [false; 10];

        while x != 0 {
            arr[x % 10] = true;
            x /= 10;
        }

        let _ = writeln!(output, "{}", arr.iter().filter(|b| **b).count());
    }

    print!("{output}");
}
use std::io::*;
use std::fmt::Write;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m, q) = next!(usize, usize, u32);
    let mut cmd = [vec![0; n], vec![0; m]];

    for (a, b, v) in (0..q).map(|_| next!(usize, usize, i32)) {
        cmd[a - 1][b - 1] += v;
    }

    for i in 0..n {
        for j in 0..m {
            let _ = write!(output, "{} ", cmd[0][i] + cmd[1][j]);
        }
        output.push('\n');
    }

    print!("{output}");
}
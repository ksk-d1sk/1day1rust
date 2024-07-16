use std::fmt::Write;
use std::io::*;

const INF: u32 = 1_000_000_000;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, u32);
    let mut edge = vec![vec![INF; n]; n];

    for (a, b, c) in (0..m).map(|_| next!(usize, usize, u32)) {
        edge[a - 1][b - 1] = edge[a - 1][b - 1].min(c);
    }

    for i in 0..n {
        edge[i][i] = 0;
    }

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                edge[j][k] = edge[j][k].min(edge[j][i] + edge[i][k]);
            }
        }
    }

    for v in edge {
        for x in v {
            let _ = if x == INF {
                write!(output, "0 ")
            } else {
                write!(output, "{x} ")
            };
        }
        output.push('\n');
    }

    print!("{output}");
}
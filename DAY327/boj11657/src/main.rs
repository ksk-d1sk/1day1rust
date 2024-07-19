use std::fmt::Write;
use std::io::*;

const INF: i64 = i64::MAX;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, u16);
    let edge = Vec::from_iter((0..m).map(|_| next!(usize, usize, i64)));
    let mut dist = vec![INF; n + 1];
    let mut cycle_check = false;

    dist[1] = 0;

    for i in 1..=n {
        for &(cur_node, next_node, w) in edge.iter() {
            if dist[cur_node] != INF {
                let next_w = dist[cur_node] + w;
                if next_w < dist[next_node] {
                    dist[next_node] = next_w;
                    if i == n {
                        cycle_check = true;
                    }
                }
            }
        }
    }

    if cycle_check {
        output.push_str("-1");
    } else {
        for i in 2..=n {
            let _ = if dist[i] == INF {
                writeln!(output, "-1")
            } else {
                writeln!(output, "{}", dist[i])
            };
        }
    }

    print!("{output}");
}
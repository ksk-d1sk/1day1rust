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

    let t = next!(u8);
    for _ in 0..t {
        let (n, m, w) = next!(usize, u16, u16);
        let mut edge = Vec::new();
        let mut dist = vec![i32::MAX as i64; n];
        let mut cycle_check = false;

        for (s, e, t) in (0..m).map(|_| next!(usize, usize, i64)) {
            edge.push((s - 1, e - 1, t));
            edge.push((e - 1, s - 1, t));
        }

        for (s, e, t) in (0..w).map(|_| next!(usize, usize, i64)) {
            edge.push((s - 1, e - 1, -t));
        }

        dist[0] = 0;

        for i in 1..=n {
            for &(cur_node, next_node, w) in edge.iter() {
                let next_w = dist[cur_node] + w;
                if next_w < dist[next_node] {
                    dist[next_node] = next_w;
                    if i == n {
                        cycle_check = true;
                    }
                }
            }
        }

        let _ = if cycle_check {
            writeln!(output, "YES")
        } else {
            writeln!(output, "NO")
        };
    }

    print!("{output}");
}
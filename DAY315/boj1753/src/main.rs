use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io::*;

const INF: u32 = u32::MAX;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (v, e) = next!(usize, usize);
    let k = next!(usize);
    let mut edge = vec![Vec::new(); v];
    let mut dist = vec![INF; v];
    let mut bh = BinaryHeap::new();

    for (u, v, w) in (0..e).map(|_| next!(usize, usize, u32)) {
        edge[u - 1].push((v - 1, w));
    }

    dist[k - 1] = 0;
    bh.push(Reverse((0, k - 1)));

    while let Some(Reverse((cur_w, cur_node))) = bh.pop() {
        for &(next_node, w) in &edge[cur_node] {
            let next_w = cur_w + w;
            if next_w < dist[next_node] {
                dist[next_node] = next_w;
                bh.push(Reverse((next_w, next_node)));
            }
        }
    }

    for x in dist {
        let _ = if x == INF {
            writeln!(output, "INF")
        } else {
            writeln!(output, "{x}")
        };
    }

    print!("{output}");
}
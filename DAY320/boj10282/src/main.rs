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

    let t = next!(u8);
    for _ in 0..t {
        let (n, d, c) = next!(usize, u32, usize);
        let mut edge = vec![Vec::new(); n];

        for (a, b, s) in (0..d).map(|_| next!(usize, usize, u32)) {
            edge[b - 1].push((a - 1, s));
        }

        let result = solve(n, c - 1, &edge);
        let _ = writeln!(output, "{} {}", result.0, result.1);
    }

    print!("{output}");
}

fn solve(n: usize, c: usize, edge: &[Vec<(usize, u32)>]) -> (usize, u32) {
    let mut pq = BinaryHeap::new();
    let mut dist = vec![INF; n];
    let mut count = 0;
    let mut last_time = 0;

    pq.push(Reverse((0, c)));
    dist[c] = 0;

    while let Some(Reverse((cur_w, cur_node))) = pq.pop() {
        if cur_w <= dist[cur_node] {
            count += 1;
            last_time = cur_w;
        }

        for &(next_node, w) in edge[cur_node].iter() {
            let next_w = cur_w + w;
            if next_w < dist[next_node] {
                dist[next_node] = next_w;
                pq.push(Reverse((next_w, next_node)));
            }
        }
    }

    (count, last_time)
}
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::*;

const INF: u32 = 9999999;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, e) = next!(usize, u32);
    let mut edge = vec![Vec::new(); n];

    for (u, v, w) in (0..e).map(|_| next!(usize, usize, u32)) {
        edge[u - 1].push((v - 1, w));
        edge[v - 1].push((u - 1, w));
    }

    let (v1, v2) = next!(usize, usize);
    let (v1, v2) = (v1 - 1, v2 - 1);

    let a = dijkstra(n, &edge, 0);
    let b = dijkstra(n, &edge, v1);
    let c = dijkstra(n, &edge, v2);

    let answer = (a[v1] + b[v2] + c[n - 1]).min(a[v2] + c[v1] + b[n - 1]);

    if answer >= INF {
        print!("-1");
    } else {
        print!("{answer}");
    }
}

fn dijkstra(n: usize, edge: &[Vec<(usize, u32)>], start: usize) -> Vec<u32> {
    let mut bh = BinaryHeap::new();
    let mut dist = vec![INF; n];

    bh.push(Reverse((0, start)));
    dist[start] = 0;

    while let Some(Reverse((cur_w, cur_node))) = bh.pop() {
        for &(next_node, w) in edge[cur_node].iter() {
            let next_w = cur_w + w;
            if next_w < dist[next_node] {
                dist[next_node] = next_w;
                bh.push(Reverse((next_w, next_node)));
            }
        }
    }

    dist
}
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::*;

const INF: u32 = u32::MAX;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, u32);
    let mut edge = vec![Vec::new(); n];
    let mut dist = vec![INF; n];
    let mut bh = BinaryHeap::new();

    for (u, v, w) in (0..m).map(|_| next!(usize, usize, u32)) {
        edge[u - 1].push((v - 1, w));
    }

    let (s, e) = next!(usize, usize);

    bh.push(Reverse((0, s - 1)));
    dist[s - 1] = 0;

    while let Some(Reverse((cur_w, cur_node))) = bh.pop() {
        if cur_node + 1 == e {
            break;
        }

        for &(next_node, w) in edge[cur_node].iter() {
            let next_w = cur_w + w;
            if next_w < dist[next_node] {
                dist[next_node] = next_w;
                bh.push(Reverse((next_w, next_node)));
            }
        }
    }

    print!("{}", dist[e - 1]);
}
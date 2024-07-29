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
    let mut edge = vec![Vec::new(); n + 1];
    let mut dist = vec![INF; n + 1];
    let mut pq = BinaryHeap::new();

    for (a, b, c) in (0..m).map(|_| next!(usize, usize, u32)) {
        edge[a].push((b, c));
        edge[b].push((a, c));
    }

    let (s, t) = next!(usize, usize);

    dist[s] = 0;
    pq.push(Reverse((0, s)));

    while let Some(Reverse((cur_w, cur_node))) = pq.pop() {
        if cur_w > dist[cur_node] {
            continue;
        } else if cur_node == t {
            break;
        }

        for &(next_node, w) in edge[cur_node].iter() {
            let next_w = cur_w + w;
            if next_w < dist[next_node] {
                dist[next_node] = next_w;
                pq.push(Reverse((next_w, next_node)));
            }
        }
    }

    print!("{}", dist[t]);
}
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

    let (n, m, x) = next!(usize, u16, usize);
    let mut from_x_edge = vec![Vec::new(); n];
    let mut to_x_edge = vec![Vec::new(); n];

    for (a, b, t) in (0..m).map(|_| next!(usize, usize, u32)) {
        from_x_edge[a - 1].push((b - 1, t));
        to_x_edge[b - 1].push((a - 1, t));
    }

    let mut answer = 0;
    let from_x_dist = dijkstra(n, x - 1, &from_x_edge);
    let to_x_dist = dijkstra(n, x - 1, &to_x_edge);

    for i in 0..n {
        answer = answer.max(from_x_dist[i] + to_x_dist[i]);
    }

    print!("{answer}");
}

fn dijkstra(n: usize, x: usize, edge: &[Vec<(usize, u32)>]) -> Vec<u32> {
    let mut dist = vec![INF; n];
    let mut pq = BinaryHeap::new();

    dist[x] = 0;
    pq.push(Reverse((0, x)));

    while let Some(Reverse((cur_w, cur_node))) = pq.pop() {
        if cur_w > dist[cur_node] {
            continue;
        }

        for &(next_node, w) in edge[cur_node].iter() {
            let next_w = cur_w + w;
            if next_w < dist[next_node] {
                dist[next_node] = next_w;
                pq.push(Reverse((next_w, next_node)));
            }
        }
    }

    dist
}
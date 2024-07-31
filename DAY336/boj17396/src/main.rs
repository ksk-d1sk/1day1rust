use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::*;

const INF: i64 = i64::MAX;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, u32);
    let visit = Vec::from_iter((0..n).map(|_| next!() == "1"));
    let mut pq = BinaryHeap::new();
    let mut edge = vec![Vec::new(); n];
    let mut dist = vec![INF; n];
    let mut answer = -1;

    for (a, b, t) in (0..m).map(|_| next!(usize, usize, i64)) {
        edge[a].push((b, t));
        edge[b].push((a, t));
    }

    dist[0] = 0;
    pq.push(Reverse((0, 0)));

    while let Some(Reverse((cur_t, cur_node))) = pq.pop() {
        if cur_node + 1 == n {
            answer = cur_t;
            break;
        } else if visit[cur_node] || cur_t > dist[cur_node] {
            continue;
        }

        for &(next_node, t) in edge[cur_node].iter() {
            let next_t = cur_t + t;
            if next_t < dist[next_node] {
                dist[next_node] = next_t;
                pq.push(Reverse((next_t, next_node)));
            }
        }
    }

    print!("{answer}");
}
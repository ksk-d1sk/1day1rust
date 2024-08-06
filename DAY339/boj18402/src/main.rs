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

    let (n, e) = next!(usize, usize);
    let t = next!(u32);
    let m = next!(u32);
    let mut edge = vec![Vec::new(); n + 1];
    let mut dist = vec![INF; n + 1];
    let mut pq = BinaryHeap::new();
    let mut answer = 0;

    for (a, b, w) in (0..m).map(|_| next!(usize, usize, u32)) {
        edge[b].push((a, w));
    }

    dist[e] = 0;
    pq.push(Reverse((0, e)));

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

    for i in 1..=n {
        if dist[i] <= t {
            answer += 1;
        }
    }

    print!("{answer}");
}
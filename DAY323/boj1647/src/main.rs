use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, u32);
    let mut edge = vec![Vec::new(); n];
    let mut visited = vec![false; n];
    let mut pq = BinaryHeap::new();
    let mut answer = 0;
    let mut max_w = 0;

    for (a, b, c) in (0..m).map(|_| next!(usize, usize, u32)) {
        edge[a - 1].push((b - 1, c));
        edge[b - 1].push((a - 1, c));
    }

    pq.push(Reverse((0, 0)));

    while let Some(Reverse((cur_w, cur_node))) = pq.pop() {
        if visited[cur_node] {
            continue;
        }

        answer += cur_w;
        max_w = max_w.max(cur_w);
        visited[cur_node] = true;

        for &(next_node, next_w) in edge[cur_node].iter() {
            pq.push(Reverse((next_w, next_node)));
        }
    }

    answer -= max_w;

    print!("{answer}");
}
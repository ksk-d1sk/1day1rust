use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io::*;

const INF: u32 = u32::MAX;
const ROOT: usize = usize::MAX;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, u32);
    let mut edge = vec![Vec::new(); n];
    let mut dist = vec![INF; n];
    let mut parents = vec![0; n];
    let mut pq = BinaryHeap::new();
    let mut stack = Vec::new();

    for (a, b, w) in (0..m).map(|_| next!(usize, usize, u32)) {
        edge[a - 1].push((b - 1, w));
    }

    let (start, end) = next!(usize, usize);
    let (start, end) = (start - 1, end - 1);

    pq.push(Reverse((0, start)));
    dist[start] = 0;
    parents[start] = ROOT;

    while let Some(Reverse((cur_w, cur_node))) = pq.pop() {
        if cur_w > dist[cur_node] {
            continue;
        }

        for &(next_node, w) in edge[cur_node].iter() {
            let next_w = cur_w + w;
            if next_w < dist[next_node] {
                dist[next_node] = next_w;
                parents[next_node] = cur_node;
                pq.push(Reverse((next_w, next_node)));
            }
        }
    }

    let mut p = end;
    while p != ROOT {
        stack.push(p + 1);
        p = parents[p];
    }

    let _ = writeln!(output, "{}\n{}", dist[end], stack.len());

    while let Some(x) = stack.pop() {
        let _ = write!(output, "{x} ");
    }

    print!("{output}");
}
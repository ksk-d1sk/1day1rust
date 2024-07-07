use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m, k, x) = next!(usize, u32, usize, usize);
    let mut answer = BinaryHeap::new();
    let mut queue = VecDeque::new();
    let mut edge = vec![Vec::new(); n];
    let mut visit = vec![true; n];

    for (a, b) in (0..m).map(|_| next!(usize, usize)) {
        edge[a - 1].push(b - 1);
    }

    queue.push_back((x - 1, 0));
    visit[x - 1] = false;

    while let Some((cur, cnt)) = queue.pop_front() {
        if cnt == k {
            answer.push(Reverse(cur + 1));
            continue;
        }

        for &next_node in edge[cur].iter() {
            if visit[next_node] {
                visit[next_node] = false;
                queue.push_back((next_node, cnt + 1));
            }
        }
    }

    if answer.is_empty() {
        let _ = write!(output, "-1");
    };

    while let Some(Reverse(x)) = answer.pop() {
        let _ = writeln!(output, "{x}");
    }

    print!("{output}");
}
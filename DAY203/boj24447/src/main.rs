use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let r = next!(usize);
    let mut edge = vec![Vec::new(); n + 1];
    let mut queue = VecDeque::new();
    let mut visit = vec![-1; n];
    let mut answer = 0_i64;
    let mut count = 2;

    for (u, v) in (0..m).map(|_| next!(usize, usize)) {
        edge[u].push(v);
        edge[v].push(u);
    }

    queue.push_back((r, 1));
    visit[r - 1] = 0;

    while let Some((node, depth)) = queue.pop_front() {
        let v = edge.get_mut(node).unwrap();
        v.sort_unstable();

        for next_node in v {
            if visit[*next_node - 1] == -1 {
                visit[*next_node - 1] = depth;
                answer += count * depth;
                count += 1;
                queue.push_back((*next_node, depth + 1));
            }
        }
    }

    print!("{answer}");
}
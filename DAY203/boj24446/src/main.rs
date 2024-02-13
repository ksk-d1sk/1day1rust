use std::collections::VecDeque;
use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let r = next!(usize);
    let mut edge = vec![Vec::new(); n + 1];
    let mut queue = VecDeque::new();
    let mut visit = vec![-1; n];

    for (u, v) in (0..m).map(|_| next!(usize, usize)) {
        edge[u].push(v);
        edge[v].push(u);
    }

    queue.push_back((r, 1));
    visit[r - 1] = 0;

    while let Some((node, cnt)) = queue.pop_front() {
        let v = edge.get_mut(node).unwrap();

        for next_node in v {
            if visit[*next_node - 1] == -1 {
                visit[*next_node - 1] = cnt;
                queue.push_back((*next_node, cnt + 1));
            }
        }
    }

    for cnt in visit {
        let _ = writeln!(output, "{cnt}");
    }

    print!("{output}");
}
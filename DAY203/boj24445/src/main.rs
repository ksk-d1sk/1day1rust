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
    let mut visit = vec![0_u32; n];
    let mut count = 2;

    for (u, v) in (0..m).map(|_| next!(usize, usize)) {
        edge[u].push(v);
        edge[v].push(u);
    }

    queue.push_back(r);
    visit[r - 1] = 1;

    while let Some(node) = queue.pop_front() {
        let v = edge.get_mut(node).unwrap();
        v.sort_unstable_by(|a, b| b.cmp(a));

        for next_node in v {
            if visit[*next_node - 1] == 0 {
                visit[*next_node - 1] = count;
                count += 1;
                queue.push_back(*next_node);
            }
        }
    }

    for cnt in visit {
        let _ = writeln!(output, "{cnt}");
    }

    print!("{output}");
}
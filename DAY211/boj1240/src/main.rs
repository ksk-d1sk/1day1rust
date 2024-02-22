use std::collections::VecDeque;
use std::fmt::Write;
use std::io::*;

const MAX: usize = 1_001;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let mut edge = vec![Vec::new(); MAX];
    let mut visit = [(0, 0); MAX];
    let mut queue = VecDeque::new();

    for (u, v, l) in (1..n).map(|_| next!(usize, usize, u32)) {
        edge[u].push((v, l));
        edge[v].push((u, l));
    }

    for i in 1..=m {
        let (s, e) = next!(usize, usize);

        queue.push_back((s, 0));
        visit[s] = (0, i);
    
        while let Some((x, l)) = queue.pop_front() {
            if x == e {
                queue.clear();
            } else {
                for &(nx, nl) in edge[x].iter() {
                    if visit[nx].1 < i {
                        visit[nx].0 = nl + l;
                        visit[nx].1 = i;
                        queue.push_back((nx, nl + l));
                    }
                }
            }
        }

        let _ = writeln!(output, "{}", visit[e].0);
    }

    print!("{output}");
}
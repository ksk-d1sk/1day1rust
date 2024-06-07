use std::io::*;

fn dfs(cur: usize, parent: u32, colors: &[u32], adj: &[Vec<usize>], visit: &mut [bool]) -> usize {
    let mut ret = 0;
    visit[cur] = false;

    if parent != colors[cur] {
        ret += 1;
    }

    for next in adj[cur].iter().copied() {
        if visit[next] {
            ret += dfs(next, colors[cur], colors, adj, visit);
        }
    }

    ret
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let colors = Vec::from_iter((0..n).map(|_| next!(u32)));
    let mut adj = vec![Vec::new(); n];
    let mut visit = vec![true; n];

    for (a, b) in (1..n).map(|_| next!(usize, usize)) {
        adj[a - 1].push(b - 1);
        adj[b - 1].push(a - 1);
    }

    print!("{}", dfs(0, 0, &colors, &adj, &mut visit));
}
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
    let mut visit = vec![-1; n];
    let mut answer = 0;
    let mut count = 1;

    for (u, v) in (0..m).map(|_| next!(usize, usize)) {
        edge[u].push(v);
        edge[v].push(u);
    }

    for v in edge.iter_mut() {
        v.sort_unstable();
    }

    dfs(&mut visit, &edge, r, 0, &mut count, &mut answer);

    print!("{answer}");
}

fn dfs(visit: &mut [i64], edge: &[Vec<usize>], i: usize, depth: i64, cnt: &mut i64, ans: &mut i64) {
    visit[i - 1] = depth;
    *ans += *cnt * depth;

    for ni in &edge[i] {
        if visit[*ni - 1] == -1 {
            *cnt += 1;
            dfs(visit, edge, *ni, depth + 1, cnt, ans);
        }
    }
}
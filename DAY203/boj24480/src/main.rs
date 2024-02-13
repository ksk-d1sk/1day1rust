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
    let mut visit = vec![0_u32; n];
    let mut count = 1;

    for (u, v) in (0..m).map(|_| next!(usize, usize)) {
        edge[u].push(v);
        edge[v].push(u);
    }

    for v in edge.iter_mut() {
        v.sort_unstable_by(|a, b| b.cmp(a));
    }

    dfs(&mut visit, &edge, r, &mut count);

    for cnt in visit {
        let _ = writeln!(output, "{cnt}");
    }

    print!("{output}");
}

fn dfs(visit: &mut [u32], edge: &[Vec<usize>], i: usize, cnt: &mut u32) {
    visit[i - 1] = *cnt;
    *cnt += 1;

    for ni in &edge[i] {
        if visit[*ni - 1] == 0 {
            dfs(visit, edge, *ni, cnt);
        }
    }
}
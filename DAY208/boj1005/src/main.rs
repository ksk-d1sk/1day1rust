use std::fmt::Write;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(usize);
    for _ in 0..t {
        let (n, k) = next!(usize, u32);
        let time = Vec::from_iter((0..n).map(|_| next!(i32)));
        let mut max = vec![-1; n];
        let mut edge = vec![Vec::new(); n];

        for (u, v) in (0..k).map(|_| next!(usize, usize)) {
            edge[v - 1].push(u - 1);
        }

        let w = next!(usize);
        
        let _ = writeln!(output, "{}", dfs(w - 1, &edge, &time, &mut max));
    }

    print!("{output}");
}

fn dfs(cur: usize, edge: &[Vec<usize>], time: &[i32], max: &mut [i32]) -> i32 {
    for &i in edge[cur].iter() {
        if max[i] == -1 {
            max[cur] = dfs(i, edge, time, max).max(max[cur]);
        } else {
            max[cur] = max[cur].max(time[i] + max[i]);
        }
    }

    if edge[cur].is_empty() {
        time[cur]
    } else {
        time[cur] + max[cur]
    }
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut edge = vec![Vec::new(); n];
    let mut visit = vec![false; n];

    for _ in 0..n {
        let a = next!(usize);
        loop {
            let b = next!(i32);

            if b == -1 {
                break;
            }

            let b = b as usize;
            let w = next!(u32);
            edge[a - 1].push((b - 1, w));
        }
    }

    let result = dfs(&edge, &mut visit, 0);

    print!("{}", result.0.max(result.1));
}

fn dfs(edge: &[Vec<(usize, u32)>], visit: &mut [bool], x: usize) -> (u32, u32) {
    let mut m1 = 0;
    let mut m2 = 0;
    let mut first = 0;
    let mut second = 0;

    visit[x] = true;

    for &(nx, w) in edge[x].iter() {
        if visit[nx] {
            continue;
        }

        let result = dfs(edge, visit, nx);
        let nw = result.0 + w;

        m1 = m1.max(nw);
        m2 = m2.max(result.1);

        if nw > first {
            second = first;
            first = nw;
        } else if nw > second {
            second = nw;
        }
    }

    m2 = m2.max(first + second);

    (m1, m2)
}
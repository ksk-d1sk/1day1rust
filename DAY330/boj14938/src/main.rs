use std::io;

const INF: u32 = 123_456_789;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m, r) = next!(usize, u32, u8);
    let t = Vec::from_iter((0..n).map(|_| next!(u32)));
    let mut edge = vec![vec![INF; n]; n];
    let mut answer = 0;

    for (a, b, l) in (0..r).map(|_| next!(usize, usize, u32)) {
        edge[a - 1][b - 1] = l;
        edge[b - 1][a - 1] = l;
    }

    for i in 0..n {
        edge[i][i] = 0;
    }

    for r in 0..n {
        for i in 0..n {
            for j in 0..n {
                edge[i][j] = edge[i][j].min(edge[i][r] + edge[r][j]);
            }
        }
    }

    for v in edge {
        let mut sum = 0;

        for i in 0..n {
            if v[i] <= m {
                sum += t[i];
            }
        }

        answer = answer.max(sum);
    }

    print!("{answer}");
}
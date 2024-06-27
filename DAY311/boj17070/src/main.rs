use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let grid: Vec<Vec<_>> = (0..n)
        .map(|_| (0..n).map(|_| next!() == "0").collect())
        .collect();
    let mut pipe = vec![vec![(0, 0, 0); n]; n];

    pipe[0][1].0 = 1_u32;

    for i in 0..n {
        for j in (0..n).filter(|&j| grid[i][j]) {
            let horizontal = j + 1 < n && grid[i][j + 1];
            let vertical = i + 1 < n && grid[i + 1][j];
            let diagonal = horizontal && vertical && grid[i + 1][j + 1];

            if pipe[i][j].0 > 0 {
                if horizontal {
                    pipe[i][j + 1].0 += pipe[i][j].0;
                }

                if diagonal {
                    pipe[i + 1][j + 1].1 += pipe[i][j].0;
                }
            }

            if pipe[i][j].1 > 0 {
                if vertical {
                    pipe[i + 1][j].2 += pipe[i][j].1;
                }

                if horizontal {
                    pipe[i][j + 1].0 += pipe[i][j].1;
                }

                if diagonal {
                    pipe[i + 1][j + 1].1 += pipe[i][j].1;
                }
            }

            if pipe[i][j].2 > 0 {
                if vertical {
                    pipe[i + 1][j].2 += pipe[i][j].2;
                }

                if diagonal {
                    pipe[i + 1][j + 1].1 += pipe[i][j].2;
                }
            }
        }
    }

    let temp = pipe[n - 1][n - 1];

    print!("{}", temp.0 + temp.1 + temp.2);
}
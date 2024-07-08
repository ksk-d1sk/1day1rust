use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io::*;

const INF: u16 = u16::MAX;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut bh = BinaryHeap::new();

    for case in 1.. {
        let n = next!(usize);

        if n == 0 {
            break;
        }

        let mut dist = vec![vec![INF; n]; n];
        let grid: Vec<Vec<_>> = (0..n)
            .map(|_| (0..n).map(|_| next!(u16)).collect())
            .collect();

        bh.push(Reverse((grid[0][0], 0, 0)));
        dist[0][0] = grid[0][0];

        while let Some(Reverse((cur_w, x, y))) = bh.pop() {
            if x + 1 == n && y + 1 == n {
                bh.clear();
                break;
            }

            for (nx, ny) in [
                (x + 1, y), (x, y + 1),
                (x - 1, y), (x, y - 1),
            ] {
                if nx < n && ny < n {
                    let next_w = cur_w + grid[nx][ny];
                    if next_w < dist[nx][ny] {
                        dist[nx][ny] = next_w;
                        bh.push(Reverse((next_w, nx, ny)));
                    }
                }
            }
        }

        let _ = writeln!(output, "Problem {case}: {}", dist[n - 1][n - 1]);
    }

    print!("{output}");
}
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::*;

const INF: u32 = u32::MAX;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut dist = vec![vec![INF; n]; n];
    let mut bh = BinaryHeap::new();
    let grid: Vec<Vec<_>> = (0..n)
        .map(|_| (0..n).map(|_| next!(u32)).collect())
        .collect();

    bh.push(Reverse((0, 0, 0)));
    dist[0][0] = 0;

    while let Some(Reverse((cur_w, x, y))) = bh.pop() {
        if x + 1 == n && y + 1 == n {
            break;
        }

        for (nx, ny) in [(x + 1, y), (x, y + 1)] {
            if nx < n && ny < n {
                let next_w = cur_w + (grid[nx][ny] + 1).saturating_sub(grid[x][y]);
                if next_w < dist[nx][ny] {
                    dist[nx][ny] = next_w;
                    bh.push(Reverse((next_w, nx, ny)));
                }
            }
        }
    }

    print!("{}", dist[n - 1][n - 1]);
}
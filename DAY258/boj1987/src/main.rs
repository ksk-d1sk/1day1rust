use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (r, c) = next!(usize, usize);
    let mut visit1 = vec![vec![true; c]; r];
    let mut visit2 = [true; 26];
    let grid: Vec<Vec<usize>> = (0..r)
        .map(|_| next!().bytes().map(|b| (b - b'A').into()).collect())
        .collect();

    print!("{}", dfs(0, 0, &grid, &mut visit1, &mut visit2))
}

fn dfs(x: usize, y: usize, grid: &[Vec<usize>], visit1: &mut [Vec<bool>], visit2: &mut [bool]) -> usize {
    let mut max = 0;
    let n = grid.len();
    let m = grid[0].len();

    visit1[x][y] = false;
    visit2[grid[x][y]] = false;

    for (nx, ny) in [
        (x + 1, y), (x, y + 1),
        (x - 1, y), (x, y - 1),
    ] {
        if nx < n && ny < m && visit1[nx][ny] && visit2[grid[nx][ny]] {
            max = dfs(nx, ny, grid, visit1, visit2).max(max);
        }
    }

    visit1[x][y] = true;
    visit2[grid[x][y]] = true;

    max + 1
}
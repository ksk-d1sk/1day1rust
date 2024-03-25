use std::io::*;
use std::iter::repeat;

trait Execute {
    fn exec(&self, f: impl FnMut()) -> bool;
}

impl Execute for bool {
    fn exec(&self, mut f: impl FnMut()) -> bool {
        if *self {
            f();
            true
        } else {
            false
        }
    }
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let grid: Vec<Vec<_>> = (0..n)
        .map(|_| next!().bytes().map(|b| b == b'*').collect())
        .collect();
    let mut x = 0;
    let mut y = 0;
    let mut waist_x = 0;
    let mut ans = (0, 0, 0, 0, 0);

    (0..n).flat_map(|i| repeat(i).take(n)).zip((0..n).cycle()).any(|(i, j)| grid[i][j].exec(|| (x, y) = (i + 1, j)));
    (0..n).any(|i| grid[x][i].exec(|| ans.0 = y - i));
    (0..n).rev().any(|i| grid[x][i].exec(|| ans.1 = i - y));
    (0..n).rev().any(|i| grid[i][y].exec(|| {ans.2 = i - x; waist_x = i}));
    (0..n).rev().any(|i| grid[i][y - 1].exec(|| ans.3 = i - waist_x));
    (0..n).rev().any(|i| grid[i][y + 1].exec(|| ans.4 = i - waist_x));

    print!(
        "{} {}\n{} {} {} {} {}",
        x + 1,
        y + 1,
        ans.0,
        ans.1,
        ans.2,
        ans.3,
        ans.4
    );
}
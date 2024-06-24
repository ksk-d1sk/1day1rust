use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m, k) = next!(usize, usize, usize);
    let mut grid = vec![vec![false; m]; n];
    let mut answer = 0;
    let mut queue = VecDeque::new();

    for (r, c) in (0..k).map(|_| next!(usize, usize)) {
        grid[r - 1][c - 1] = true;
    }

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] {
                let mut area = 0;

                queue.push_back((i, j));
                grid[i][j] = false;

                while let Some((x, y)) = queue.pop_front() {
                    area += 1;

                    for (nx, ny) in [
                        (x + 1, y), (x, y + 1),
                        (x - 1, y), (x, y - 1),
                    ] {
                        if nx < n && ny < m && grid[nx][ny] {
                            grid[nx][ny] = false;
                            queue.push_back((nx, ny));
                        }
                    }
                }

                answer = answer.max(area);
            }
        }
    }

    print!("{answer}");
}
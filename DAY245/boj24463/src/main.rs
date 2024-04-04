use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let mut queue1 = VecDeque::new();
    let mut queue2 = VecDeque::new();
    let mut exit = (0, 0);
    let mut depth_grid = vec![vec![0; m]; n];
    let mut grid: Vec<Vec<_>> = (0..n)
        .map(|r| {
            next!()
                .chars()
                .enumerate()
                .map(|(c, ch)| {
                    if (r == 0 || r == n - 1) && ch == '.'
                    || (c == 0 || c == m - 1) && ch == '.'
                    {
                        if queue1.is_empty() {
                            depth_grid[r][c] = 1;
                            queue1.push_back((r, c));
                        } else {
                            queue2.push_back((r, c));
                            exit = (r, c);
                        }
                    }

                    if ch == '.' {
                        '@'
                    } else {
                        ch
                    }
                })
                .collect()
        })
        .collect();

    while let Some((x, y)) = queue1.pop_front() {
        if (x, y) == exit {
            break;
        }

        for (nx, ny) in [
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ] {
            if nx < n && ny < m && grid[nx][ny] == '@' && depth_grid[nx][ny] == 0 {
                depth_grid[nx][ny] = depth_grid[x][y] + 1;
                queue1.push_back((nx, ny));
            }
        }
    }

    while let Some((x, y)) = queue2.pop_front() {
        grid[x][y] = '.';

        if depth_grid[x][y] == 1 {
            break;
        }

        for (nx, ny) in [
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ] {
            if nx < n && ny < m && depth_grid[x][y] - 1 == depth_grid[nx][ny] {
                queue2.push_back((nx, ny));
            }
        }
    }

    for v in grid {
        for ch in v {
            output.push(ch);
        }
        output.push('\n');
    }

    print!("{output}");
}
use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let mut visited = vec![vec![false; m]; n];
    let mut queue = VecDeque::new();
    let mut answer = 0;
    let grid: Vec<Vec<_>> = (0..n)
        .map(|_| (0..m).map(|_| next!(u8)).collect())
        .collect();

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 0 && !visited[i][j] {
                answer += 1;
                queue.push_back((i, j));

                while let Some((x, y)) = queue.pop_front() {
                    for (mut nx, mut ny) in [
                        (x + 1, y), (x, y + 1),
                        (x - 1, y), (x, y - 1),
                    ] {
                        if nx > n {
                            nx = n - 1;
                        } else if ny > m {
                            ny = m - 1
                        } else if nx == n{
                            nx = 0;
                        } else if ny == m {
                            ny = 0;
                        }

                        if grid[nx][ny] == 0 && !visited[nx][ny] {
                            visited[nx][ny] = true;
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
        }
    }

    print!("{answer}");
}
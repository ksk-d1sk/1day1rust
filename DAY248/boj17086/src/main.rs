use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = 0;
    let (n, m) = next!(usize, usize);
    let mut queue = VecDeque::new();
    let mut grid: Vec<Vec<_>> = (0..n)
        .map(|r| {
            (0..m)
                .map(|c| {
                    let temp = next!(u8);
                    if temp == 1 {
                        queue.push_back((r, c, 0));
                    }
                    temp
                })
                .collect()
        })
        .collect();

    while let Some((x, y, cnt)) = queue.pop_front() {
        answer = cnt;

        for (nx, ny) in [
            (x + 1, y + 1), (x + 1, y - 1),
            (x - 1, y + 1), (x - 1, y - 1),
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ] {
            if nx < n && ny < m && grid[nx][ny] == 0 {
                grid[nx][ny] = 1;
                queue.push_back((nx, ny, cnt + 1));
            }
        }
    }

    print!("{answer}");
}
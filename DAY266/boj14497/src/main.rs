use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let (x1, y1) = next!(usize, usize);
    let (x2, y2) = next!(usize, usize);
    let mut answer = 0;
    let mut deque = VecDeque::new();
    let mut visit = vec![vec![true; m]; n];
    let grid: Vec<Vec<_>> = (0..n)
        .map(|_| next!().bytes().collect())
        .collect();

    deque.push_back((x1 - 1, y1 - 1, 1));
    visit[x1 - 1][y1 - 1] = false;

    while let Some((x, y, cnt)) = deque.pop_front() {
        if x + 1 == x2 && y + 1 == y2 {
            answer = cnt;
            break;
        }

        for (nx, ny) in [
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ] {
            if nx < n && ny < m && visit[nx][ny] {
                visit[nx][ny] = false;
                if grid[nx][ny] == b'1' {
                    deque.push_back((nx, ny, cnt + 1));
                } else {
                    deque.push_front((nx, ny, cnt));
                }
            }
        }
    }

    print!("{answer}");
}
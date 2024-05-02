use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut answer = 0;
    let mut deque = VecDeque::new();
    let mut visit = vec![vec![true; n]; n];
    let grid: Vec<Vec<_>> = (0..n)
        .map(|_| next!().bytes().collect())
        .collect();

    deque.push_back((0, 0, 0));

    while let Some((x, y, cnt)) = deque.pop_front() {
        if x + 1 == n && y + 1 == n {
            answer = cnt;
            break;
        }

        for (nx, ny) in [
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ] {
            if nx < n && ny < n && visit[nx][ny] {
                visit[nx][ny] = false;
                if grid[nx][ny] == b'1' {
                    deque.push_front((nx, ny, cnt));
                } else {
                    deque.push_back((nx, ny, cnt + 1));
                }
            }
        }
    }

    print!("{answer}");
}
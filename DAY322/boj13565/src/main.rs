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
    let mut answer = false;
    let mut queue = VecDeque::new();
    let mut visit = vec![vec![true; m]; n];
    let grid: Vec<_> = (0..n)
        .map(|_| next!().as_bytes())
        .collect();

    for i in 0..m {
        if grid[0][i] == b'0' {
            visit[0][i] = false;
            queue.push_back((0, i));
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        if x + 1 == n {
            answer = true;
            break;
        }

        for (nx, ny) in [
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ] {
            if nx < n && ny < m && grid[nx][ny] == b'0' && visit[nx][ny] {
                visit[nx][ny] = false;
                queue.push_back((nx, ny));
            }
        }
    }

    if answer {
        print!("YES");
    } else {
        print!("NO");
    }
}
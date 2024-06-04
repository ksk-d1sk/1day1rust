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
    let mut queue = VecDeque::new();
    let mut visit = vec![vec![true; m]; n];
    let mut answer = false;
    let grid: Vec<Vec<_>> = (0..n)
        .map(|_| (0..m).map(|_| next!()).collect())
        .collect();
    let jump = next!(usize);

    queue.push_back((0, 0));
    visit[0][0] = false;

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == (n - 1, m - 1) {
            answer = true;
            break;
        }

        for i in 1..=jump {
            for j in 0..=i {
                let k = i - j;
                for (nx, ny) in [
                    (x + j, y + k), (x + j, y - k),
                    (x - j, y + k), (x - j, y - k),
                ] {
                    if nx < n && ny < m && visit[nx][ny] && grid[nx][ny] == grid[0][0] {
                        visit[nx][ny] = false;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }

    if answer {
        print!("ALIVE");
    } else {
        print!("DEAD");
    }
}
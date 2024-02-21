use std::collections::VecDeque;
use std::io::*;

use Object::*;

enum Object {
    Visit(usize, u8), // (i번째 방문, 벽을 부순 횟수)
    Empty,
    Wall(u8),
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m, k) = next!(usize, usize, u8);
    let mut grid: Vec<Vec<_>> = (0..n)
        .map(|_|
            next!()
                .bytes()
                .map(|b| if b == b'0' { Empty } else { Wall(u8::MAX) })
                .collect()
        )
        .collect();
    let mut queue = VecDeque::new();

    queue.push_back((0, 0, 1, 0));
    grid[0][0] = Visit(1, 0);

    while let Some((x, y, cnt, break_cnt)) = queue.pop_front() {
        if x == n - 1 && y == m - 1 {
            break;
        }

        for (nx, ny) in [
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ] {
            if nx < n && ny < m {
                match grid[nx][ny] {
                    Visit(_, brc) if break_cnt < brc => {
                        grid[nx][ny] = Visit(cnt + 1, break_cnt);
                        queue.push_back((nx, ny, cnt + 1, break_cnt));
                    },
                    Empty => {
                        grid[nx][ny] = Visit(cnt + 1, break_cnt);
                        queue.push_back((nx, ny, cnt + 1, break_cnt));
                    },
                    Wall(brc) if break_cnt < k && break_cnt < brc => {
                        grid[nx][ny] = Wall(break_cnt);
                        queue.push_back((nx, ny, cnt + 1, break_cnt + 1));
                    },
                    _ => {},
                }
            }
        }
    }

    match grid[n - 1][m - 1] {
        Visit(answer, _) => print!("{answer}"),
        Empty => print!("-1"),
        Wall(_) => unreachable!(),
    }
}
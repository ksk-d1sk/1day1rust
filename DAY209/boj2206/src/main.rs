use std::collections::VecDeque;
use std::io;

use Object::*;

#[derive(PartialEq)]
enum Object {
    Visit((i32, bool)), // (i번째 방문, 벽을 부수고 방문한 것인지 여부)
    Empty,
    Wall,
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let mut grid: Vec<Vec<_>> = (0..n)
        .map(|_| {
            next!().bytes().map(|b| 
                if b == b'0' {
                    Empty
                } else {
                    Wall
                }
            )
            .collect()
        })
        .collect();
    let mut queue = VecDeque::new();

    queue.push_back((0, 0, 1, true));
    grid[0][0] = Visit((1, false));

    while let Some((x, y, cnt, can_break)) = queue.pop_front() {
        if x == n - 1 && y == m - 1 { break }

        for (nx, ny) in [
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
         ] {
            if nx < n && ny < m {
                if can_break && grid[nx][ny] == Wall {
                    queue.push_back((nx, ny, cnt + 1, false));
                } else if grid[nx][ny] == Empty {
                    grid[nx][ny] = Visit((cnt + 1, !can_break));
                    queue.push_back((nx, ny, cnt + 1, can_break));
                } else if let Visit((_, true)) = grid[nx][ny] {
                    if can_break {
                        grid[nx][ny] = Visit((cnt + 1, !can_break));
                        queue.push_back((nx, ny, cnt + 1, can_break));
                    }
                }
            }
        }
    }

    match grid[n - 1][m - 1] {
        Visit((answer, _)) => print!("{answer}"),
        Empty => print!("-1"),
        Wall => unreachable!(),
    }
}
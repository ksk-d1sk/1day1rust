use std::collections::VecDeque;
use std::io::*;

use Object::*;

enum Object {
    O(usize),
    X,
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let mut queue = VecDeque::new();
    let mut o = Vec::new();
    let mut borders = Vec::new();
    let mut areas = vec![0];
    let mut answer = 0;

    let mut grid: Vec<Vec<_>> = (0..n)
        .map(|r| {
            (0..m)
                .map(|c| {
                    if next!(u8) == 1 {
                        o.push((r, c));
                        O(0)
                    } else {
                        X
                    }
                })
                .collect()
        })
        .collect();

    while let Some((r, c)) = o.pop() {
        if let O(0) = grid[r][c] {
            let id = areas.len();
            let mut area = 1_u32;

            grid[r][c] = O(id);
            queue.push_back((r, c));

            while let Some((x, y)) = queue.pop_front() {
                for (nx, ny) in [
                    (x + 1, y), (x, y + 1),
                    (x - 1, y), (x, y - 1),
                ] {
                    if nx < n && ny < m {
                        match grid[nx][ny] {
                            O(0) => {
                                area += 1;
                                grid[nx][ny] = O(id);
                                queue.push_back((nx, ny));
                            }
                            X => borders.push((nx, ny, id)),
                            _ => {}
                        }
                    }
                }
            }

            areas.push(area);
        }
    }

    while let Some((x, y, id)) = borders.pop() {
        let mut new_area = 1 + areas[id];
        let mut visit = vec![id];

        for (nx, ny) in [
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ] {
            if nx < n && ny < m {
                if let O(other) = grid[nx][ny] {
                    if !visit.contains(&other) {
                        new_area += areas[other];
                        visit.push(other);
                    }
                }
            }
        }

        answer = answer.max(new_area);
    }

    print!("{answer}");
}
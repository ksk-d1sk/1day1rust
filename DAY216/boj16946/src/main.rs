use std::collections::{VecDeque, HashSet};
use std::io::*;

#[derive(Debug)]
enum State {
    Wall(u8),
    Empty(bool),
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let mut grid: Vec<Vec<_>> = (0..n)
        .map(|_|
            next!()
                .bytes()
                .map(|b| if b == b'1' { State::Wall(1) } else { State::Empty(false) })
                .collect()
        )
        .collect();
    let mut queue = VecDeque::new();
    let mut adjacency_wall = HashSet::new();

    for i in 0..n {
        for j in 0..m {
            if let State::Empty(false) = grid[i][j] {
                let mut area = 0;

                queue.push_back((i, j));
                grid[i][j] = State::Empty(true);

                while let Some((x, y)) = queue.pop_front() {
                    area += 1;

                    for (nx, ny) in [
                        (x + 1, y), (x, y + 1),
                        (x - 1, y), (x, y - 1),
                    ] {
                        if nx < n && ny < m {
                            match grid[nx][ny] {
                                State::Empty(false) => {
                                    queue.push_back((nx, ny));
                                    grid[nx][ny] = State::Empty(true);
                                },
                                State::Wall(_) => {
                                    adjacency_wall.insert((nx, ny));
                                },
                                _ => {},
                            }
                        }
                    }
                }

                for &(x, y) in &adjacency_wall {
                    if let State::Wall(count) = &mut grid[x][y] {
                        *count = (*count + (area % 10) as u8) % 10;
                    }
                }
                adjacency_wall.clear();
            }
        }
    }

    for v in grid {
        for state in v {
            if let State::Wall(count) = state {
                output.push((count + b'0') as char);
            } else {
                output.push('0');
            }
        }
        output.push('\n');
    }

    print!("{output}");
}

use std::collections::VecDeque;
use std::io::*;

enum Object {
    Ice(u8, usize),
    Water(usize),
}

impl Object {
    fn sub(&mut self, turn: usize) -> bool {
        if let Self::Ice(size, _) = self {
            *size -= 1;

            if size == &0 {
                *self = Object::Water(turn);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = 0;
    let (n, m) = next!(usize, usize);
    let mut queue = VecDeque::new();
    let mut ice = Vec::new();
    let mut ice_count = 0_usize;
    let mut turn = 0;

    let mut grid: Vec<Vec<_>> = (0..n)
        .map(|r|
            (0..m)
                .map(|c| match next!(u8) {
                    0 => Object::Water(0),
                    size => {
                        ice_count += 1;

                        if ice.is_empty() {
                            ice.push((r, c));
                        }

                        Object::Ice(size, 0)
                    }
                })
                .collect()
        )
        .collect();

    while let Some((r, c)) = ice.pop() {
        let mut area = 0;
        let mut melted_ice_count = 0;

        turn += 1;
        queue.push_back((r, c));

        if let Object::Ice(_, visit) = &mut grid[r][c] {
            *visit = turn;
        }

        while let Some((x, y)) = queue.pop_front() {
            area += 1;

            for (nx, ny) in [
                (x + 1, y), (x, y + 1),
                (x - 1, y), (x, y - 1),
            ] {
                if nx < n && ny < m {
                    match &mut grid[nx][ny] {
                        Object::Ice(_, visit) if *visit < turn => {
                            *visit = turn;
                            queue.push_back((nx, ny));
                        },
                        Object::Water(visit) if *visit < turn => {
                            if grid[x][y].sub(turn) {
                                melted_ice_count += 1;
                            }
                        },
                        _ => {},
                    }
                }
            }

            if ice.is_empty() {
                if let Object::Ice(_, _) = grid[x][y] {
                    ice.push((x, y));
                }
            }
        }

        if area != ice_count {
            answer = turn - 1;
            ice.clear();
        } else {
            ice_count -= melted_ice_count;
        }
    }

    print!("{answer}");
}
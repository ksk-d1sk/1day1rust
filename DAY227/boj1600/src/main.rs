use std::collections::VecDeque;
use std::io::*;

enum Object {
    Empty,
    Visit(u8),
    Wall,
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let k = next!(u8);
    let (w, h) = next!(usize, usize);
    let mut answer = -1;
    let mut queue = VecDeque::new();

    let mut grid: Vec<Vec<_>> = (0..h)
        .map(|_| {
            (0..w)
                .map(|_| if next!(u8) == 1 {
                    Object::Wall
                } else {
                    Object::Empty
                })
                .collect()
        })
        .collect();

    queue.push_back((0, 0, 0, k));
    grid[0][0] = Object::Visit(k);

    while let Some((x, y, visit, mk)) = queue.pop_front() {
        if x == h - 1 && y == w - 1 {
            answer = visit;
            break;
        }

        let h_movement: &[(usize, usize)] = &[
            (x - 2, y - 1), (x - 2, y + 1),
            (x + 2, y - 1), (x + 2, y + 1),
            (x + 1, y - 2), (x - 1, y - 2),
            (x + 1, y + 2), (x - 1, y + 2),
        ];
        let m_movement: &[(usize, usize)] = &[
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ];

        for (d_arr, nmk) in [(h_movement, true), (m_movement, false)]
            .iter()
            .filter(|&(_, is_horse)| !is_horse || mk != 0)
            .map(|&(d_arr, is_horse)| if is_horse {
                (d_arr, mk - 1)
            } else {
                (d_arr, mk)
            }) 
        {
            for &(nx, ny) in d_arr {
                if nx < h && ny < w {
                    match grid[nx][ny] {
                        Object::Empty => {
                            grid[nx][ny] = Object::Visit(nmk);
                            queue.push_back((nx, ny, visit + 1, nmk));
                        }
                        Object::Visit(vk) if vk < nmk => {
                            grid[nx][ny] = Object::Visit(nmk);
                            queue.push_back((nx, ny, visit + 1, nmk));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    print!("{answer}");
}
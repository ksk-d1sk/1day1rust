use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();

    let (mut start, mut end) = ((0, 0), (0, 0));
    let mut map: Vec<Vec<_>> = input
        .skip(2)
        .enumerate()
        .map(|(r, row)| {
            row.char_indices()
                .map(|(c, ch)| match ch {
                    '#' => -1,
                    '.' => i32::MAX,
                    'S' => {
                        start = (r as i32, c as i32);
                        0
                    }
                    'E' => {
                        end = (r as i32, c as i32);
                        i32::MAX
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut queue = VecDeque::from([(start, 0)]);

    while let Some(((r, c), time)) = queue.pop_front() {
        let new_time = time + 1;

        for (adj_r, adj_c) in get_adjacents(r, c) {
            let adj = (adj_r as usize, adj_c as usize);

            if map[adj.0][adj.1] == -1 {
                continue;
            }

            if is_adjacent_to_wall(r, c, &map) && is_adjacent_to_wall(adj_r, adj_c, &map) {
                if (adj_r, adj_c) == end {
                    println!("{time}");
                    return;
                }

                if map[adj.0][adj.1] <= time {
                    continue;
                }

                map[adj.0][adj.1] = time;
                queue.push_front(((adj_r, adj_c), time));
            } else {
                if (adj_r, adj_c) == end {
                    println!("{new_time}");
                    return;
                }

                if map[adj.0][adj.1] <= new_time {
                    continue;
                }

                map[adj.0][adj.1] = new_time;
                queue.push_back(((adj_r, adj_c), new_time));
            }
        }
    }
}

fn get_adjacents(r: i32, c: i32) -> [(i32, i32); 4] {
    [(r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1)]
}

fn is_adjacent_to_wall(r: i32, c: i32, map: &[Vec<i32>]) -> bool {
    get_adjacents(r, c)
        .iter()
        .any(|&(adj_r, adj_c)| map[adj_r as usize][adj_c as usize] == -1)
}

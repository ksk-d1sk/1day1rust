use std::collections::VecDeque;
use std::fmt::Write;
use std::io::*;

enum Object {
    Visit(usize),
    Empty,
    Wall,
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m, p) = next!(usize, usize, usize);
    let mut answer = vec![0; p + 1];
    let mut vec_queue = vec![VecDeque::new(); p + 1];
    let mut temp_queue = VecDeque::new();
    let mut vec_move = vec![0];

    vec_move.extend((0..p).map(|_| next!(u32)));

    let mut grid: Vec<Vec<_>> = (0..n)
        .map(|i| {
            next!()
                .bytes()
                .enumerate()
                .map(|(j, b)| match b {
                    b'.' => Object::Empty,
                    b'#' => Object::Wall,
                    id => {
                        let id = (id - b'0') as usize;
                        vec_queue[id].push_back((i, j, 1));
                        Object::Visit(id)
                    }
                })
                .collect()
        })
        .collect();

    while vec_queue.iter().any(|q| !q.is_empty()) {
        for id in 1..=p {
            while let Some((x, y, cnt)) = vec_queue[id].pop_front() {
                answer[id] += 1;

                for (nx, ny) in [
                    (x + 1, y), (x, y + 1),
                    (x - 1, y), (x, y - 1),
                ] {
                    if nx < n && ny < m {
                        if let Object::Empty = grid[nx][ny] {
                            grid[nx][ny] = Object::Visit(id);
    
                            if cnt == vec_move[id] {
                                temp_queue.push_back((nx, ny, 1));
                            } else {
                                vec_queue[id].push_back((nx, ny, cnt + 1));
                            }
                        }
                    }
                }
            }

            vec_queue[id].append(&mut temp_queue);
        }
    }

    for x in &answer[1..] {
        let _ = write!(output, "{x} ");
    }

    print!("{output}");
}
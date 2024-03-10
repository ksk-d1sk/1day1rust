use std::collections::VecDeque;
use std::io::*;

#[derive(Debug)]
enum Object {
    Hedgehog(usize),
    Water(usize),
    Empty,
    Exit,
    Rock,
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut result = Err("KAKTUS");
    let (r, c) = next!(usize, usize);
    let mut q1 = VecDeque::new();
    let mut q2 = VecDeque::new();

    let mut grid: Vec<Vec<_>> = (0..r)
        .map(|i|
            next!()
                .bytes()
                .enumerate()
                .map(|(j, b)| match b {
                    b'.' => Object::Empty,
                    b'*' => {
                        q1.push_back((i, j, 0));
                        Object::Water(0)
                    },
                    b'X' => Object::Rock,
                    b'D' => Object::Exit,
                    b'S' => {
                        q2.push_back((i, j, 0));
                        Object::Hedgehog(0)
                    },
                    _ => unreachable!(),
                })
                .collect()
        )
        .collect();

    while let Some((x, y, cnt)) = q1.pop_front() {
        for (nx, ny) in [
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ] {
            if nx < r && ny < c {
                match grid[nx][ny] {
                    Object::Empty => {
                        grid[nx][ny] = Object::Water(cnt + 1);
                        q1.push_back((nx, ny, cnt + 1));
                    },
                    _ => {},
                }
            }
        }
    }

    'a: while let Some((x, y, cnt)) = q2.pop_front() {
        for (nx, ny) in [
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ] {
            if nx < r && ny < c {
                match grid[nx][ny] {
                    Object::Empty => {
                        grid[nx][ny] = Object::Hedgehog(cnt + 1);
                        q2.push_back((nx, ny, cnt + 1));
                    },
                    Object::Water(visit) if cnt + 1 < visit => {
                        grid[nx][ny] = Object::Hedgehog(cnt + 1);
                        q2.push_back((nx, ny, cnt + 1));
                    },
                    Object::Exit => {
                        result = Ok(cnt + 1);
                        break 'a;
                    },
                    _ => {},
                }
            }
        }
    }

    match result {
        Ok(answer) => print!("{answer}"),
        Err(msg) => print!("{msg}"),
    }
}
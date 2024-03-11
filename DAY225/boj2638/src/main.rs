use std::collections::VecDeque;
use std::io::*;

enum Object {
    Cheese(usize, u8),
    Air(usize),
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let mut cheese = 0;

    let mut grid: Vec<Vec<_>> = (0..n)
        .map(|_|
            (0..m)
                .map(|_| if next!(u8) == 1 {
                    cheese += 1;
                    Object::Cheese(0, 0)
                } else {
                    Object::Air(0)
                })
                .collect()
        )
        .collect();

    let mut time = 0;
    let mut queue = VecDeque::new();

    while cheese != 0 {
        if let Object::Air(t) = &mut grid[0][0] {
            time += 1;
            *t = time;
            queue.push_back((0, 0));
        }

        while let Some((x, y)) = queue.pop_front() {
            for (nx, ny) in [
                (x + 1, y), (x, y + 1),
                (x - 1, y), (x, y - 1),
            ] {
                if nx < n && ny < m {
                    match &mut grid[nx][ny] {
                        Object::Air(t) if *t < time => {
                            *t = time;
                            queue.push_back((nx, ny));
                        }
                        Object::Cheese(t, cnt) if *t < time => {
                            *t = time;
                            *cnt = 1;
                        }
                        Object::Cheese(t, cnt) if *t == time => {
                            *cnt += 1;

                            if *cnt == 2 {
                                grid[nx][ny] = Object::Air(time);
                                cheese -= 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    print!("{time}");
}
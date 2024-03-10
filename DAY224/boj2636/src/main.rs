use std::collections::VecDeque;
use std::io::*;

enum Object {
    Cheese,
    Empty(usize),
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let mut queue = VecDeque::new();
    let mut v = vec![usize::MAX];
    let mut time = 0;

    let mut grid: Vec<Vec<_>> = (0..n)
        .map(|_|
            (0..m)
                .map(|_| if next!(u8) == 1 {
                    Object::Cheese
                } else {
                    Object::Empty(0)
                })
                .collect()
        )
        .collect();

    while v[time] != 0 {
        time += 1;
        v.push(0);
        queue.push_back((0, 0));

        while let Some((x, y)) = queue.pop_front() {
            for (nx, ny) in [
                (x + 1, y), (x, y + 1),
                (x - 1, y), (x, y - 1),
            ] {
                if nx < n && ny < m {
                    match &mut grid[nx][ny] {
                        Object::Cheese => {
                            v[time] += 1;
                            grid[nx][ny] = Object::Empty(time);
                        },
                        Object::Empty(visit) if *visit < time => {
                            *visit = time;
                            queue.push_back((nx, ny));
                        },
                        _ => {},
                    }
                }
            }
        }
    }

    print!("{}\n{}", time - 1, v[time - 1]);
}
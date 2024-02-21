use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut answer = 1;
    let mut grid = [[(0, 0); 100]; 100];
    let mut min = u8::MAX;
    let mut max = 0;

    for i in 0..n {
        for (j, h) in (0..n).map(|j| (j, next!(u8))) {
            min = min.min(h);
            max = max.max(h);
            grid[i][j].0 = h;
        }
    }

    let mut queue = VecDeque::new();

    for h in min..max {
        let mut count = 0;

        for i in 0..n {
            for j in 0..n {
                if h < grid[i][j].0 && grid[i][j].1 != h {
                    count += 1;
                    grid[i][j].1 = h;
                    queue.push_back((i, j));
                    bfs(n, &mut queue, &mut grid, h);
                }
            }
        }

        answer = answer.max(count);
    }

    print!("{answer}");
}

fn bfs(n: usize, q: &mut VecDeque<(usize, usize)>, grid: &mut [[(u8, u8); 100]], h: u8) {
    while let Some((x, y)) = q.pop_front() {
        for (nx, ny) in [
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ] {
            if nx < n && ny < n && h < grid[nx][ny].0 && grid[nx][ny].1 != h {
                grid[nx][ny].1 = h;
                q.push_back((nx, ny));
            }
        }
    }
}
use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let mut grid = vec![vec![false; n]; n];
    let mut switchs = vec![vec![Vec::new(); n]; n];
    let mut visit = vec![vec![false; n]; n];
    let mut darkness_visit = vec![vec![false; n]; n];
    let mut queue = VecDeque::new();
    let mut answer = 1;

    for (x, y, a, b) in (0..m).map(|_| next!(usize, usize, usize, usize)) {
        let (x, y, a, b) = (x - 1, y - 1, a - 1, b - 1);
        switchs[x][y].push((a, b));
    }

    queue.push_back((0, 0));
    grid[0][0] = true;
    visit[0][0] = true;

    while let Some((x, y)) = queue.pop_front() {
        for &(sw_x, sw_y) in &switchs[x][y] {
            if !grid[sw_x][sw_y] {
                answer += 1;
                grid[sw_x][sw_y] = true;
            }

            if darkness_visit[sw_x][sw_y] && !visit[sw_x][sw_y] {
                queue.push_back((sw_x, sw_y));
            }
        }

        for (nx, ny) in [
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ] {
            if nx < n && ny < n && !visit[nx][ny] {
                if grid[nx][ny] {
                    visit[nx][ny] = true;
                    queue.push_back((nx, ny));
                } else {
                    darkness_visit[nx][ny] = true;
                }
            }
        }
    }

    print!("{answer}");
}
use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let nm = n * m;
    let mut answer = 0;
    let mut max_safety_area = nm;
    let mut queue = VecDeque::new();
    let mut virus = Vec::new();
    let grid: Vec<Vec<_>> = (0..n)
        .map(|r|
            (0..m)
                .map(|c| {
                    let temp = next!(u8);
                    if temp == 2 {
                        virus.push((r, c));
                    } else if temp == 1 {
                        max_safety_area -= 1;
                    }
                    temp
                })
                .collect()
        )
        .collect();

    for i in (0..nm-2).filter(|&i| grid[i / m][i % m] == 0) {
        for j in (i+1..nm-1).filter(|&i| grid[i / m][i % m] == 0) {
            for k in (j+1..nm).filter(|&i| grid[i / m][i % m] == 0) {
                let mut safety_area = max_safety_area - 3;
                let mut visit = [[true; 8]; 8];

                for &(x, y) in virus.iter() {
                    visit[x][y] = false;
                    queue.push_back((x, y));
                }

                while let Some((x, y)) = queue.pop_front() {
                    safety_area -= 1;
                    for (nx, ny) in [
                        (x + 1, y), (x, y + 1),
                        (x - 1, y), (x, y - 1),
                    ] {
                        let p = nx * m + ny;
                        if nx < n && ny < m && visit[nx][ny] && grid[nx][ny] != 1 && p != i && p != j && p != k {
                            visit[nx][ny] = false;
                            queue.push_back((nx, ny));
                        }
                    }
                }

                answer = answer.max(safety_area);
            }
        }
    }

    print!("{answer}");
}
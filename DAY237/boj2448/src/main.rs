use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut queue = VecDeque::new();
    let mut temp_queue = VecDeque::new();
    let mut heap = BinaryHeap::new();
    let mut shark_size = 2;
    let mut shark_exp = 0;
    let mut visit = [[false; 20]; 20];
    let mut answer = 0;

    let mut grid: Vec<Vec<_>> = (0..n)
        .map(|r| {
            (0..n)
                .map(|c| match next!(u8) {
                    9 => {
                        visit[r][c] = true;
                        queue.push_back((r, c, 0));
                        0
                    }
                    x => x,
                })
                .collect()
        })
        .collect();

    while !queue.is_empty() {
        let mut time = 0;

        while let Some((x, y, move_count)) = queue.pop_front() {
            if grid[x][y] != 0 && shark_size > grid[x][y] {
                time = move_count;
                heap.push(Reverse((x, y)));
            }

            for (nx, ny) in [
                (x + 1, y), (x, y + 1),
                (x - 1, y), (x, y - 1),
            ] {
                if nx < n && ny < n && !visit[nx][ny] && shark_size >= grid[nx][ny] {
                    visit[nx][ny] = true;
                    temp_queue.push_back((nx, ny, move_count + 1));
                }
            }
        }

        (queue, temp_queue) = (temp_queue, queue);

        if let Some(fish) = heap.pop() {
            let (x, y) = fish.0;

            shark_exp += 1;
            grid[x][y] = 0;
            answer += time;
            visit = [[false; 20]; 20];

            heap.clear();
            queue.clear();

            queue.push_back((x, y, 0));

            if shark_exp == shark_size {
                shark_exp = 0;
                shark_size += 1;
            }
        }
    }

    print!("{answer}");
}
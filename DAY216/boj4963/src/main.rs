use std::collections::VecDeque;
use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut grid = [[0; 50]; 50];
    let mut queue = VecDeque::new();
    let mut next_search = Vec::new();

    loop {
        let mut answer = 0;
        let (w, h) = next!(usize, usize);

        if w + h == 0 {
            break;
        }

        for i in 0..h {
            for j in 0..w {
                let temp = next!(u8);

                if temp == 1 {
                    next_search.push((i, j));
                }

                grid[i][j] = temp;
            }
        }

        while let Some((r, c)) = next_search.pop() {
            if grid[r][c] == 1 {
                answer += 1;
                queue.push_back((r, c));
                grid[r][c] = 0;

                while let Some((x, y)) = queue.pop_front() {
                    for (nx, ny) in [
                        (x - 1, y - 1), (x - 1, y), (x - 1, y + 1),
                        (x    , y - 1),             (x    , y + 1),
                        (x + 1, y - 1), (x + 1, y), (x + 1, y + 1),
                    ] {
                        if nx < h && ny < w && grid[nx][ny] == 1 {
                            queue.push_back((nx, ny));
                            grid[nx][ny] = 0;
                        }
                    }
                }
            }
        }

        let _ = writeln!(output, "{answer}");
    }

    print!("{output}");
}
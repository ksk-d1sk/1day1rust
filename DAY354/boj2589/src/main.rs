use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (l, w) = next!(usize, usize);
    let cheese: Vec<_> = (0..l).map(|_| next!().as_bytes()).collect();
    let mut answer = 0_u16;

    for i in 0..l {
        for j in 0..w {
            if cheese[i][j] == b'L' {
                let mut queue = VecDeque::from([(i, j, 0)]);
                let mut visit = vec![vec![true; w]; l];

                visit[i][j] = false;

                while let Some((x, y, cnt)) = queue.pop_front() {
                    answer = answer.max(cnt);
                    for (nx, ny) in [
                        (x + 1, y), (x, y + 1),
                        (x - 1, y), (x, y - 1),
                    ] {
                        if nx < l && ny < w && visit[nx][ny] && cheese[nx][ny] == b'L' {
                            visit[nx][ny] = false;
                            queue.push_back((nx, ny, cnt + 1));
                        }
                    }
                }
            }
        }
    }

    print!("{answer}");
}
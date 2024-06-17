use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);
    let danger_zone = Vec::from_iter((0..n).map(|_| next!(usize, usize, usize, usize)));
    let m = next!(u8);
    let dead_zone = Vec::from_iter((0..m).map(|_| next!(usize, usize, usize, usize)));
    let mut answer = -1;
    let mut deque = VecDeque::new();
    let mut visit = vec![vec![false; 501]; 501];

    deque.push_back((0, 0, 0));
    visit[0][0] = true;

    while let Some((x, y, cnt)) = deque.pop_front() {
        if (x, y) == (500, 500) {
            answer = cnt;
            break;
        }

        'a: for (nx, ny) in [
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ] {
            if nx > 500 || ny > 500 || visit[nx][ny] {
                continue;
            }

            let mut danger_zone_check = false;

            for (x1, y1, x2, y2) in dead_zone.iter().copied() {
                let (max_x, min_x) = if x1 > x2 { (x1, x2) } else { (x2, x1) };
                let (max_y, min_y) = if y1 > y2 { (y1, y2) } else { (y2, y1) };
                if min_x <= nx && nx <= max_x && min_y <= ny && ny <= max_y {
                    continue 'a;
                }
            }

            for (x1, y1, x2, y2) in danger_zone.iter().copied() {
                let (max_x, min_x) = if x1 > x2 { (x1, x2) } else { (x2, x1) };
                let (max_y, min_y) = if y1 > y2 { (y1, y2) } else { (y2, y1) };
                if min_x <= nx && nx <= max_x && min_y <= ny && ny <= max_y {
                    danger_zone_check = true;
                    break;
                }
            }

            visit[nx][ny] = true;
            if danger_zone_check {
                deque.push_back((nx, ny, cnt + 1));
            } else {
                deque.push_front((nx, ny, cnt));
            }
        }
    }

    print!("{answer}");
}
use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    // let input = read_to_string(std::fs::File::open("input.txt").unwrap()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let limit_t = next!(i8);
    let mut tree = vec![[true; 21]];
    let mut deque = VecDeque::new();
    let mut answer = -1;

    tree.extend(
        (0..n)
            .map(|_| {
                let m = next!(u8);
                let mut ret = [false; 21];
                for i in (0..m).map(|_| next!(usize)) {
                    ret[i] = true;
                }
                ret
            })
    );

    deque.push_back((0, 1, 0));

    while let Some((x, y, cnt)) = deque.pop_front() {
        if x == n {
            answer = cnt;
            break;
        }

        if !tree[x][y] {
            continue;
        }

        let nx = x + 1;
        tree[x][y] = false;
        for ny in [y, y + 1, y - 1, (y * 2).min(20)] {
            if ny <= 20 && tree[nx][ny] {
                deque.push_front((nx, ny, cnt));
            }
        }

        for ny in 1..=20 {
            if tree[nx][ny] && cnt < limit_t {
                deque.push_back((nx, ny, cnt + 1));
            }
        }
    }

    print!("{answer}");
}
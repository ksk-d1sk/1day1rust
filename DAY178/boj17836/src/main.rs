use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m, t) = get!(usize, usize, usize);
    let mut gram = (0, 0);
    let mut castle: Vec<Vec<_>> = (0..n)
        .map(|i| (0..m)
            .map(|j| {
                let temp = get!(u8);
                if temp == 2 { gram = (i, j) }
                temp
            })
            .collect()
        )
        .collect();
    let mut gram_count = usize::MAX;
    let mut princess_count = usize::MAX;
    let mut queue = VecDeque::new();

    queue.push_back((0, 0, 0));
    castle[0][0] = 1;

    while let Some((x, y, cnt)) = queue.pop_front() {
        if x == n - 1 && y == m - 1 {
            princess_count = cnt;
            break;
        }
        if x == gram.0 && y == gram.1 {
            gram_count = cnt + n + m - gram.0 - gram.1 - 2;
        }

        for (nx, ny) in [
            (x - 1, y), (x, y - 1),
            (x + 1, y), (x, y + 1),
        ] {
            if nx < n && ny < m && castle[nx][ny] != 1 {
                castle[nx][ny] = 1;
                queue.push_back((nx, ny, cnt + 1));
            }
        }
    }

    let min_count = gram_count.min(princess_count);

    if min_count > t {
        print!("Fail");
    } else {
        print!("{min_count}");
    }
}
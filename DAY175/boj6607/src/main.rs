use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().and_then(|x| x.parse::<$t>().ok())),+) };
    }

    let mut queue = VecDeque::new();

    while let (Some(r) , Some(c),
               Some(gr), Some(gc),
               Some(lr), Some(lc)) = get!(usize, usize, usize, usize, usize, usize) {
        let mut answer = None;
        let mut visit = vec![vec![true; c]; r];

        queue.push_back((gr - 1, gc - 1, 0_usize));
        visit[gr - 1][gc - 1] = false;

        while let Some((x, y, cnt)) = queue.pop_front() {
            if x + 1 == lr && y + 1 == lc {
                answer = Some(cnt);
                queue.clear();
                break;
            }

            for (nx, ny) in [
                (x - 2, y - 1), (x - 2, y + 1),
                (x - 1, y - 2), (x + 1, y - 2),
                (x + 2, y - 1), (x + 2, y + 1),
                (x + 1, y + 2), (x - 1, y + 2),
            ] {
                if nx < r && ny < c && visit[nx][ny] {
                    queue.push_back((nx, ny, cnt + 1));
                    visit[nx][ny] = false;
                }
            }
        }

        let _ = if let Some(count) = answer {
            writeln!(output, "{count}")
        } else {
            writeln!(output, "impossible")
        };
    }

    print!("{output}");
}
use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut queue = VecDeque::new();

    for _ in 0..get!(usize) {
        let n = get!(usize);
        let (ni, nj) = get!(usize, usize);
        let (x, y) = get!(usize, usize);

        
        let mut count = usize::MAX;
        let mut visit = vec![vec![true; n]; n];
        
        queue.push_back((ni, nj, 0_usize));
        visit[ni][nj] = false;
        
        while let Some((r, c, cnt)) = queue.pop_front() {
            if r == x && c == y {
                count = cnt;
                queue.clear();
                break;
            }
    
            for (nr, nc) in [
                (r - 2, c - 1), (r - 2, c + 1),
                (r - 1, c - 2), (r + 1, c - 2),
                (r + 2, c - 1), (r + 2, c + 1),
                (r + 1, c + 2), (r - 1, c + 2),
            ] {
                if nr < n && nc < n && visit[nr][nc] {
                    queue.push_back((nr, nc, cnt + 1));
                    visit[nr][nc] = false;
                }
            }
        }
    
        let _ = writeln!(output, "{count}");
    }

    print!("{output}");
}
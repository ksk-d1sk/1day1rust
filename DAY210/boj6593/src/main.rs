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

    for _ in 0.. {
        let (l, r, c) = next!(usize, usize, usize);
        if l + r + c == 0 { break }
        let mut building = [[[0; 30]; 30]; 30];
        let mut answer = Err("Trapped!");
        let (mut sl, mut sr, mut sc) = (0, 0, 0);
        let (mut el, mut er, mut ec) = (0, 0, 0);

        for i in 0..l {
            for j in 0..r {
                for (k, b) in next!().bytes().enumerate() {
                    if b == b'S' {
                        (sl, sr, sc) = (i, j, k);
                    } else if b == b'E' {
                        (el, er, ec) = (i, j, k);
                    }
                    building[i][j][k] = b;
                }
            }
        }

        let mut queue = VecDeque::new();

        queue.push_back((sl, sr, sc, 0_usize));
        building[sl][sr][sc] = b'#';

        while let Some((x, y, z, cnt)) = queue.pop_front() {
            if x == el && y == er && z == ec {
                answer = Ok(cnt);
                break;
            }

            for (nx, ny, nz) in [
                (x + 1, y, z), (x, y + 1, z), (x, y, z + 1),
                (x - 1, y, z), (x, y - 1, z), (x, y, z - 1),
            ] {
                if nx < l && ny < r && nz < c && building[nx][ny][nz] != b'#' {
                    building[nx][ny][nz] = b'#';
                    queue.push_back((nx, ny, nz, cnt + 1));
                }
            }
        }

        let _ = match answer {
            Ok(count) => writeln!(output, "Escaped in {count} minute(s)."),
            Err(msg) => writeln!(output, "{msg}"),
        };
    }

    print!("{output}");
}
use std::io;
use std::collections::VecDeque;
use std::fmt::Write;

fn main() {
    // let use std::io::Read;
    // let mut f = std::fs::File::open("input.txt").unwrap();
    // let mut input = String::new();
    let input = io::read_to_string(io::stdin()).unwrap();
    // f.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let mut map: Vec<Vec<i32>> = Vec::with_capacity(n);
    let mut n_xy = (0, 0);

    for x in 0..n {
        map.push((0..m).map(|y| {
            let temp = get!(i32);
            if temp == 2 {
                n_xy = (x, y);
            }
            temp
        }).collect());
    }

    map[n_xy.0][n_xy.1] -= 2;

    let mut queue = VecDeque::new();
    const ROUTE: [(i8, i8); 4] = [
        (1, 0), (-1, 0),
        (0, 1), (0, -1),
    ];

    queue.push_back((n_xy.0, n_xy.1, 0));

    while let Some((x, y, c)) = queue.pop_front() {
        map[x][y] -= c;

        for (dx, dy) in ROUTE {
            let nx = x as i32 + dx as i32;
            let ny = y as i32 + dy as i32;
            if 0 <= nx && nx < n as i32 &&
               0 <= ny && ny < m as i32
            {
                let nx = nx as usize;
                let ny = ny as usize;
                if map[nx][ny] == 1 {
                    map[nx][ny] -= 1;
                    queue.push_back((nx, ny, c + 1));
                }
            }
        }
    }

    for r in map {
        for c in r {
            let _ = write!(output, "{} ", c * -1);
        }
        output.push('\n');
    }

    print!("{output}");
}

use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let mut ir = 0;
    let mut ic = 0;
    let mut visit = vec![vec![true; m]; n];
    let grid: Vec<_> = (0..n)
        .map(|r| {
            let tmp = get!().as_bytes();
            for (c, &item) in tmp.iter().enumerate() {
                if item == b'I' {
                    ir = r;
                    ic = c;
                }
            }

            tmp
        })
        .collect();

    let answer = dfs(&grid, &mut visit, ir, ic);

    if answer != 0 {
        print!("{answer}");
    } else {
        print!("TT");
    }
}

fn dfs(grid: &[&[u8]], visit: &mut [Vec<bool>], i: usize, j: usize) -> usize {
    let mut count = 0;
    visit[i][j] = false;

    for (dx, dy) in [
        (1, 0), (-1, 0),
        (0, 1), (0, -1),
    ] {
        if let (Some(nx), Some(ny)) = (i.checked_add_signed(dx), j.checked_add_signed(dy)) {
            if grid.get(nx).and_then(|x| x.get(ny)).is_some() && visit[nx][ny] {
                if grid[nx][ny] == b'P' {
                    count += 1;
                }

                if grid[nx][ny] != b'X' {
                    count += dfs(grid, visit, nx, ny);
                }
            }
        }
    }

    count
}
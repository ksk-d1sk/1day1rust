use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut board: Vec<Vec<char>> = (0..n).map(|_| get!().chars().collect()).collect();
    let mut heap = BinaryHeap::with_capacity(n * n);
    let mut house_count = 0;

    for r in 0..n {
        for c in 0..n {
            if board[r][c] == '0' { continue }

            house_count += 1;
            let result = dfs(&mut board, r, c);
            heap.push(Reverse(result));
        }
    }

    let _ = writeln!(output, "{house_count}");
    while let Some(e) = heap.pop() {
        let _ = writeln!(output, "{}", e.0);
    }

    print!("{output}");
}

fn dfs(board: &mut [Vec<char>], i: usize, j: usize) -> usize {
    if board[i][j] == '1' {
        let mut ret = 1;
        board[i][j] = '0';
    
        for (dx, dy) in [
            (1, 0), (-1, 0),
            (0, 1), (0, -1),
        ] {
            if let (Some(nx), Some(ny)) = (i.checked_add_signed(dx), j.checked_add_signed(dy)) {
                if nx < board.len() && ny < board.len() {
                    ret += dfs(board, nx, ny);
                }
            }
        }

        ret
    } else {
        0
    }
}
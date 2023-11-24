use std::io;
use std::iter::repeat;

const STAR: [[bool; 5]; 5] = [
    [false, false, true , false, false],
    [false, false, true , false, false],
    [true , true , true , true , true ],
    [false, true , true , true , false],
    [false, true , false, true , false],
];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let p = 5_usize.pow(n as u32);
    let mut board = vec![vec![' '; p]; p];

    rec(n, p / 5, 0, 0, &mut board);

    print!(
        "{}",
        board.into_iter()
            .map(String::from_iter)
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn rec(n: usize, p: usize, pr: usize, pc: usize, board: &mut [Vec<char>]) {
    (0..5)
        .flat_map(|x| repeat(x).take(5))
        .zip((0..5).cycle())
        .filter(|(r, c)| STAR[*r][*c])
        .map(|(r, c)| (r * p, c * p))
        .for_each(|(r, c)| {
            if n != 0 {
                rec(n - 1, p / 5, pr + r, pc + c, board);
            } else {
                board[pr][pc] = '*';
            }
        });
}
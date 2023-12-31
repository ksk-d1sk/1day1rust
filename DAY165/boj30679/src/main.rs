use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let matrix: Vec<Vec<_>> = (0..n)
        .map(|_| (0..m).map(|_| get!(u8)).collect())
        .collect();
    let mut count = 0;

    for i in 0..n {
        let mut visit = vec![vec![0_i8; m]; n];
        let mut check = true;
        let mut direction = 1;
        let mut r = i;
        let mut c = 0;

        while visit[r][c] & direction == 0 {
            let point = matrix[r][c] as usize;
            visit[r][c] |= direction;

            match direction {
                1 => c += point,
                2 => r += point,
                4 => c -= point,
                8 => r -= point,
                _ => unreachable!(),
            }

            if !(r < n && c < m) {
                check = false;
                break;
            }

            direction = (direction << 1) % 15;
        }

        if check {
            count += 1;
            let _ = write!(output, "{} ", i + 1);
        }
    }

    print!("{count}\n{output}");
}
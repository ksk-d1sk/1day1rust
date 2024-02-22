use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let grid: Vec<_> = (0..n).map(|_| next!().as_bytes()).collect();
    let mut r = 0;
    let mut b = 0;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == b'1' {
                r += 1;
            } else {
                b += 1;
            }
        }
    }

    print!("{}", r.min(b));
}
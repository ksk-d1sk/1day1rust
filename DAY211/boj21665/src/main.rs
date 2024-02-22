use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let mut count = 0;
    let a: Vec<_> = (0..n).map(|_| next!().as_bytes()).collect();
    let b: Vec<_> = (0..n).map(|_| next!().as_bytes()).collect();

    for i in 0..n {
        for j in 0..m {
            if a[i][j] == b[i][j] {
                count += 1;
            }
        }
    }

    print!("{count}");
}
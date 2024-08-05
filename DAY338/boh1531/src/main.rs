use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(u8, usize);
    let mut v = [[0; 101]; 101];

    for (x1, y1, x2, y2) in (0..n).map(|_| next!(usize, usize, usize, usize)) {
        for i in x1..=x2 {
            for j in y1..=y2 {
                v[i][j] += 1;
            }
        }
    }

    print!("{}", v.iter().flatten().filter(|x| **x > m).count());
}
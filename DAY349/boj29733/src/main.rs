use std::io::*;
use std::iter::repeat;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (r, c, h) = next!(usize, usize, usize);
    let mut answer = vec![vec![vec![b'0'; c]; r]; h];
    let cube: Vec<Vec<_>> = (0..h)
        .map(|_| (0..r).map(|_| next!().as_bytes()).collect())
        .collect();

    (0..h)
        .flat_map(|x| repeat(x).take(r))
        .zip((0..r).cycle())
        .flat_map(|x| repeat(x).take(c))
        .zip((0..c).cycle())
        .filter(|&((i, j), k)| cube[i][j][k] == b'*')
        .for_each(|((i, j), k)| {
            answer[i][j][k] = b'*';
            for x in [i - 1, i, i + 1] {
                for (y, z) in [
                    (j - 1, k - 1), (j - 1, k), (j - 1, k + 1),
                    (j + 0, k - 1), (j + 0, k), (j + 0, k + 1),
                    (j + 1, k - 1), (j + 1, k), (j + 1, k + 1),
                ] {
                    if x < h && y < r && z < c && cube[x][y][z] == b'.' {
                        answer[x][y][z] = (answer[x][y][z]  - b'0' + 1) % 10 + b'0';
                    }
                }
            }
        });

    answer.into_iter().flatten().for_each(|v| {
        v.into_iter().for_each(|x| output.push(x as char));
        output.push('\n');
    });

    print!("{output}");
}
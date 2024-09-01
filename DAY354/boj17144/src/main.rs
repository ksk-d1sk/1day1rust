use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (r, c, t) = next!(usize, usize, u16);
    let mut a = usize::MAX;
    let mut b = usize::MAX;
    let mut koosaga: Vec<Vec<_>> = (0..r)
        .map(|h| (0..c)
            .map(|_| {
                let temp = next!(i32);
                if temp == -1 {
                    if a == usize::MAX {
                        a = h;
                    } else {
                        b = h;
                    }
                }
                temp
            })
            .collect()
        )
        .collect();

    for _ in 0..t {
        let mut new_koosaga = vec![vec![0; c]; r];
        for i in 0..r {
            for j in 0..c {
                if koosaga[i][j] == -1 {
                    new_koosaga[i][j] = -1;
                } else {
                    let df = koosaga[i][j] / 5;
                    new_koosaga[i][j] += koosaga[i][j];
                    for (ni, nj) in [
                        (i + 1, j), (i, j + 1),
                        (i - 1, j), (i, j - 1),
                    ] {
                        if ni < r && nj < c && koosaga[ni][nj] != -1 {
                            new_koosaga[i][j] -= df;
                            new_koosaga[ni][nj] += df;
                        }
                    }
                }
            }
        }

        koosaga = new_koosaga;

        spin(&mut koosaga, a + 1, c, a, true);
        spin(&mut koosaga, r - b, c, b, false);
    }

    let answer: i32 = koosaga
        .into_iter()
        .flatten()
        .filter(|x| *x != -1)
        .sum();

    print!("{answer}");
}

fn spin(koosaga: &mut [Vec<i32>], r: usize, c: usize, h: usize, rev: bool) {
    let mut x = h;
    let mut y = 0;
    let mut temp = koosaga[x][y];
    let mv = if rev {
        [(c, 0, 1), (r, -1, 0), (c, 0, -1), (r, 1, 0)]
    } else {
        [(c, 0, 1), (r, 1, 0), (c, 0, -1), (r, -1, 0)]
    };

    for (m, dx, dy) in mv {
        for _ in 1..m {
            x = (dx + x as i16) as usize;
            y = (dy + y as i16) as usize;

            if temp == -1 {
                temp = koosaga[x][y];
                koosaga[x][y] = 0;
            } else if koosaga[x][y] == -1 {
                temp = 0;
            } else {
                (temp, koosaga[x][y]) = (koosaga[x][y], temp);
            }
        }
    }
}
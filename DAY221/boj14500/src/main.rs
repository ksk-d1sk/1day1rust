use std::io::*;

const O: bool = true;
const X: bool = false;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    // 구데기
    let tetrominoes: [&[&[bool]]; 19] = [
        &[&[O, O, O, O]],

        &[&[O],
          &[O],
          &[O],
          &[O]],

        &[&[O, O],
          &[O, O]],

        &[&[O, X],
          &[O, X],
          &[O, O]],

        &[&[O, O],
          &[O, X],
          &[O, X]],

        &[&[O, O],
          &[X, O],
          &[X, O]],

        &[&[X, O],
          &[X, O],
          &[O, O]],

        &[&[O, X, X],
          &[O, O, O]],

        &[&[O, O, O],
          &[O, X, X]],

        &[&[X, X, O],
          &[O, O, O]],

        &[&[O, O, O],
          &[X, X, O]],

        &[&[O, O, O],
          &[X, O, X]],

        &[&[X, O, X],
          &[O, O, O]],

        &[&[X, O],
          &[O, O],
          &[X, O]],

        &[&[O, X],
          &[O, O],
          &[O, X]],

        &[&[X, O],
          &[O, O],
          &[O, X]],

        &[&[O, X],
          &[O, O],
          &[X, O]],

        &[&[X, O, O],
          &[O, O, X]],

        &[&[O, O, X],
          &[X, O, O]],
    ];

    let (n, m) = next!(usize, usize);
    let mut answer = 0;
    let grid: Vec<Vec<_>> = (0..n)
        .map(|_|
            (0..m)
                .map(|_| next!(u32))
                .collect()
        )
        .collect();

    for tetromino in tetrominoes {
        for x in 0..=(n - tetromino.len()) {
            for y in 0..=(m - tetromino[0].len()) {
                let mut sum = 0;

                for i in 0..tetromino.len() {
                    for j in 0..tetromino[0].len() {
                        if tetromino[i][j] {
                            sum += grid[x + i][y + j];
                        }
                    }
                }

                answer = answer.max(sum);
            }
        }

    }

    print!("{answer}");
}
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();
    let m = 1_000_000_007;

    println!("{}", solve(n, m));
}

fn solve(mut n: u64, m: u64) -> u64 {
    let mut result = [[1, 0], [0, 1]];
    let mut metrix = [[1, 1], [1, 0]];

    while n > 0 {
        if n & 1 == 1 {
            result = metrix_mul(result, metrix, m);
        }
        metrix = metrix_mul(metrix, metrix, m);
        n >>= 1;
    }

    result[1][0]
}

fn metrix_mul(a: [[u64; 2]; 2], b: [[u64; 2]; 2], m: u64) -> [[u64; 2]; 2] {
    let mut answer = [[0, 0], [0, 0]];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                answer[i][j] += a[i][k] * b[k][j];
                answer[i][j] %= m;
            }
        }
    }

    answer
}
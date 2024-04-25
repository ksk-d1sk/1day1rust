use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
        () => { tokens.next().unwrap() };
    }

    let (a, b) = next!(u64, u64);
    let fibo_a = fibo(a - 1);
    let fibo_b = fibo(b);
    let mut answer = 1_000_000_000;

    answer += if b & 1 == 1 {
        fibo_b[0][0] + fibo_b[0][1]
    } else {
        fibo_b[0][1] + fibo_b[0][0]
    } - 1;

    answer -= if a & 1 == 1 {
        fibo_a[0][0] + fibo_a[0][1]
    } else {
        fibo_a[0][1] + fibo_a[0][0]
    } - 1;

    print!("{}", answer % 1_000_000_000);
}

fn fibo(mut n: u64) -> [[u64; 2]; 2] {
    let mut ret = [
        [1, 0],
        [0, 1],
    ];

    let mut fibo = [
        [1, 1],
        [1, 0],
    ];

    while n > 0 {
        if n & 1 == 1 {
            ret = metrix_mul(ret, fibo);
        }
        fibo = metrix_mul(fibo, fibo);
        n >>= 1;
    }

    ret
}

fn metrix_mul(a: [[u64; 2]; 2], b: [[u64; 2]; 2]) -> [[u64; 2]; 2] {
    let mut ret = [[0; 2]; 2];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                ret[i][j] += a[i][k] * b[k][j];
                ret[i][j] %= 1_000_000_000;
            }
        }
    }

    ret
}
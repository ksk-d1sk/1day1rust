use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(u64, u64);
    let x = gcd(n, m);

    print!("{}", fibo(x));
}

fn gcd(a: u64, b: u64) -> u64 {
    if a > b {
        euclidean(a, b)
    } else {
        euclidean(b, a)
    }
}

fn euclidean(a: u64, b: u64) -> u64 {
    let c = a % b;
    if c == 0 {
        b
    } else {
        euclidean(b, c)
    }
}

fn fibo(mut n: u64) -> u64 {
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

    ret[0][1]
}

fn metrix_mul(a: [[u64; 2]; 2], b: [[u64; 2]; 2]) -> [[u64; 2]; 2] {
    let mut ret = [[0; 2]; 2];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                ret[i][j] += a[i][k] * b[k][j];
                ret[i][j] %= 1_000_000_007;
            }
        }
    }

    ret
}
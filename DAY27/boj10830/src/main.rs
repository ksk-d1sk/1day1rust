use std::io::Read;
use std::fmt::Write;

fn main() {
    // let mut f = std::fs::File::open("input.txt").unwrap();
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // f.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ( $(input.next().unwrap().parse::<$t>().unwrap()),+ ) };
    }

    macro_rules! get_metrix {
        ($n:expr) => {{
            let mut metrix = Vec::with_capacity($n);
            for _ in 0..$n {
                let mut row = Vec::with_capacity($n);
                for _ in 0..$n {
                    row.push(get!(u32));
                }
                metrix.push(row);
            }
            metrix
        }};
    }

    let (n, mut b) = get!(usize, u64);
    let m = 1000;
    let mut metrix = get_metrix!(n);
    let mut answer = Vec::with_capacity(n);

    for i in 0..n {
        let mut v = Vec::with_capacity(n);
        for j in 0..n {
            v.push(if i == j { 1 } else { 0 });
        }
        answer.push(v);
    }

    while b > 0 {
        if b & 1 == 1 {
            answer = metrix_mul(&answer, &metrix, m);
        }
        metrix = metrix_mul(&metrix, &metrix, m);
        b >>= 1;
    }

    for row in answer {
        for e in row {
            let _ = write!(output, "{} ", e);
        }
        output.push('\n');
    }

    print!("{output}");
}

fn metrix_mul(a: &Vec<Vec<u32>>, b: &Vec<Vec<u32>>, m: u32) -> Vec<Vec<u32>> {
    let row_a = a.len();
    let row_b = b.len();
    let col_b = b[0].len();

    let mut answer = vec![vec![0; col_b]; row_a];

    for i in 0..row_a {
        for j in 0..col_b {
            for k in 0..row_b {
                answer[i][j] += a[i][k] * b[k][j];
                answer[i][j] %= m;
            }
        }
    }

    answer
}
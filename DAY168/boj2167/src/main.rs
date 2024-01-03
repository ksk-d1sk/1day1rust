use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let mut sum_vec: Vec<Vec<_>> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut v = Vec::with_capacity(m);

        v.push(get!(i32));

        for (i, x) in (1..m).map(|_| get!(i32)).enumerate() {
            v.push(v[i] + x);
        }

        sum_vec.push(v);
    }

    for j in 0..m {
        for i in 1..n {
            sum_vec[i][j] += sum_vec[i - 1][j];
        }
    }

    let k = get!(u16);
    for (i, j, x, y) in (0..k).map(|_| get!(usize, usize, usize, usize)) {
        let a = sum_vec[x - 1][y - 1];
        let b = *sum_vec.get(i - 2).and_then(|row| row.get(j - 2)).unwrap_or(&0);
        let c = *sum_vec.get(i - 2).and_then(|row| row.get(y - 1)).unwrap_or(&0);
        let d = *sum_vec.get(x - 1).and_then(|row| row.get(j - 2)).unwrap_or(&0);

        let _ = writeln!(output, "{}", a + b - c - d);
    }

    print!("{output}");
}
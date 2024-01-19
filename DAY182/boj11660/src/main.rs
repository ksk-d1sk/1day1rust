use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, u32);
    let mut sum_table: Vec<Vec<u32>> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut sum_v = Vec::with_capacity(n);
        sum_v.push(get!(u32));

        for i in 1..n {
            sum_v.push(sum_v[i - 1] + get!(u32));
        }

        sum_table.push(sum_v);
    }

    for i in 1..n {
        for j in 0..n {
            sum_table[i][j] += sum_table[i - 1][j];
        }
    }

    for (x1, y1, x2, y2) in (0..m).map(|_| get!(usize, usize, usize, usize)) {
        let s1 = sum_table[x2 - 1][y2 - 1];
        let s2 = sum_table.get(x1 - 2).and_then(|v| v.get(y1 - 2)).unwrap_or(&0);
        let s3 = sum_table.get(x1 - 2).and_then(|v| v.get(y2 - 1)).unwrap_or(&0);
        let s4 = sum_table.get(x2 - 1).and_then(|v| v.get(y1 - 2)).unwrap_or(&0);
        let _  = writeln!(output, "{}", s1 + s2 - s3 - s4);
    }

    print!("{output}");
}
use std::fmt::Write;
use std::io;

struct BackTracking<T> {
    n: usize,
    m: usize,
    arr: Vec<T>,
}

impl<T> BackTracking<T>
where
    T: std::fmt::Display + Copy
{
    fn new(n:usize, m: usize, arr: Vec<T>) -> Self {
        Self { n, m, arr }
    }

    fn solve(self) -> String {
        let mut output = String::new();
        self.dfs(&mut output, &mut Vec::with_capacity(self.m), 0);

        output
    }

    fn dfs(&self, output: &mut String, tmp_arr: &mut Vec<T>, i: usize) {
        if tmp_arr.len() == self.m {
            for elem in tmp_arr.iter() {
                let _ = write!(output, "{} ", elem);
            }
            output.push('\n');
        } else {
            for j in i..self.n {
                tmp_arr.push(self.arr[j]);
                self.dfs(output, tmp_arr, j);
                tmp_arr.pop();
            }
        }
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let mut v: Vec<u16> = input.flat_map(str::parse).collect();

    v.sort_unstable();

    let bk = BackTracking::new(n, m, v);

    print!("{}", bk.solve());
}
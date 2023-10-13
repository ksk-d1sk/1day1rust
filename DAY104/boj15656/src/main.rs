use std::fmt::Write;
use std::io;

struct BackTracking<T> {
    n: usize,
    m: usize,
    arr: Vec<T>,
    buffer: Vec<T>,
}

impl<T> BackTracking<T>
where
    T: Ord + std::fmt::Display + Copy
{
    fn new(n: usize, m: usize, mut arr: Vec<T>) -> Self {
        let buffer = Vec::with_capacity(m);
        arr.sort_unstable();
        Self { n, m, arr, buffer }
    }

    fn solve(mut self) -> String {
        let mut output = String::new();
        self.dfs(&mut output);

        output
    }

    fn dfs(&mut self, output: &mut String) {
        if self.buffer.len() == self.m {
            for elem in self.buffer.iter() {
                let _ = write!(output, "{} ", elem);
            }
            output.push('\n');
        } else {
            for i in 0..self.n {
                self.buffer.push(self.arr[i]);
                self.dfs(output);
                self.buffer.pop();
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
    let v: Vec<u16> = input.flat_map(str::parse).collect();

    let bk = BackTracking::new(n, m, v);

    print!("{}", bk.solve());
}
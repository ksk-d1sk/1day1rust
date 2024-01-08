use std::fmt::Write;
use std::io;

struct BackTracking<T> {
    n: usize,
    m: usize,
    arr: Vec<T>,
}

impl<T> BackTracking<T>
where
    T: std::fmt::Display + Copy + Ord
{
    fn new(n:usize, m: usize, arr: Vec<T>) -> Self {
        Self { n, m, arr }
    }

    fn solve(mut self) -> String {
        let mut output = String::new();
        let mut compress_arr = Vec::with_capacity(self.n);
        let mut cnt = 0;

        self.arr.sort_unstable();
        
        for window in self.arr.windows(2) {
            cnt += 1;

            if window[0] != window[1] {
                compress_arr.push((window[0], cnt));
                cnt = 0;
            }
        }

        compress_arr.push((self.arr[self.n - 1], cnt + 1));

        self.dfs(&mut output, &mut compress_arr, &mut Vec::with_capacity(self.m), 0);

        output
    }
    
    fn dfs(&self, output: &mut String, compress_arr: &mut [(T, usize)], temp_arr: &mut Vec<T>, depth: usize) {
        if self.m == depth {
            for elem in temp_arr.iter() {
                let _ = write!(output, "{elem} ");
            }
            output.push('\n');
        } else {
            for i in 0..compress_arr.len() {
                if compress_arr[i].1 != 0 {
                    compress_arr[i].1 -= 1;
                    temp_arr.push(compress_arr[i].0);
                    self.dfs(output, compress_arr, temp_arr, depth + 1);
                    temp_arr.pop();
                    compress_arr[i].1 += 1;
                }
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
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
        let mut unique_arr = Vec::with_capacity(self.n);

        self.arr.sort_unstable();
        for elem in self.arr.iter() {
            if unique_arr.last() != Some(elem) {
                unique_arr.push(*elem);
            }
        }
        
        self.dfs(&mut output, &unique_arr, &mut Vec::with_capacity(self.m), 0, 0);

        output
    }
    
    fn dfs(
        &self,
        output: &mut String,
        unique_arr: &[T],
        temp_arr: &mut Vec<T>,
        i: usize,
        depth: usize,
    ) {
        if self.m == depth {
            for elem in temp_arr.iter() {
                let _ = write!(output, "{elem} ");
            }
            output.push('\n');
        } else {
            for j in i..unique_arr.len() {
                temp_arr.push(unique_arr[j]);
                self.dfs(output, unique_arr, temp_arr, j, depth + 1);
                temp_arr.pop();
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
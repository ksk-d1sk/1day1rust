// (30105) 아즈버의 이빨 자국
// https://www.acmicpc.net/problem/30105
// !! 못푼 코드 정답 아님 !!

use std::fmt::Write;
// use std::io;

fn main() {
    use std::io::Read;
    let mut f = std::fs::File::open("input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    // let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let arr: Vec<u32> = input.flat_map(str::parse).collect();

    let mut answer = Vec::new();

    'a: for i in 1..=(n / 2) {
        let dif = arr[i] - arr[0];
        for j in 1..(n - i) {
            if arr[i + j] - arr[j] != dif {
                continue 'a;
            }
        }
        answer.push(dif);
    }

    let _ = writeln!(output, "{}", answer.len());
    for a in answer {
        let _ = write!(output, "{} ", a);
    }

    print!("{output}");
}
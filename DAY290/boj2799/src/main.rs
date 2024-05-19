use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }


    let (n, m) = next!(usize, usize);
    let v = Vec::from_iter((0..(n * 5 + 1)).map(|_| next!().as_bytes()));
    let mut answer = [0_usize; 5];

    for i in 0..n {
        for j in 0..m {
            let mut idx = 0;
            for k in 0..4 {
                if v[i * 5 + 1 + k][j * 5 + 1] == b'*' {
                    idx += 1;
                }
            }
            answer[idx] += 1;
        }
    }

    for ans in answer {
        let _ = write!(output, "{ans} ");
    }

    print!("{output}");
}
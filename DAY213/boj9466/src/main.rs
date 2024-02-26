use std::fmt::Write;
use std::io::*;

use State::*;

#[derive(Clone, Copy)]
enum State {
    Wait(usize),
    Progress(usize),
    Complete,
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(usize);
    let mut arr = vec![Complete; 100_001];

    for _ in 0..t {
        let n = next!(usize);
        let mut answer = 0;

        for i in 1..=n {
            arr[i] = Wait(next!(usize));
        }

        for i in 1..=n {
            if let Wait(_) = arr[i] {
                let (count, _, _) = dfs(i, &mut arr);
                answer += count;
            }
        }

        let _ = writeln!(output, "{answer}");
    }

    print!("{output}");
}

fn dfs(i: usize, arr: &mut [State]) -> (u32, usize, bool) {
    match arr[i] {
        Wait(j) => {
            arr[i] = Progress(i);
            let (cnt, k, chk) = dfs(j, arr);
            arr[i] = Complete;

            if chk {
                (0, k, i != k)
            } else {
                (cnt + 1, k, false)
            }
        },
        Progress(j) => {
            (0, j, i == j)
        },
        Complete => (0, i, false),
    }
}
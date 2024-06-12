use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let v = Vec::from_iter((0..n-1).map(|_| next!(usize)));
    let mut visit = vec![0; n + 1];
    let mut answer = 0;

    visit[0] = usize::MAX;

    for i in 1..n {
        let mut x = i;
        let mut check = true;

        for j in 0..n {
            if x <= n && i > visit[x] {
                if j + 1 != n {
                    visit[x] = i;
                    x = v[j] - x;
                }
            } else {
                check = false;
                break;
            }
        }

        if check {
            answer = i;
            break;
        }
    }

    for i in 0..n {
        let _ = write!(output, "{answer} ");
        answer = v.get(i).unwrap_or(&0) - answer;
    }

    print!("{output}");
}
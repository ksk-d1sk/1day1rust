// 제 풀이가 틀렸나봐요 '^'

use std::io::*;

use State::*;

#[derive(Clone)]
enum State {
    Dust,
    Clean(u8),
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (h, w) = next!(usize, usize);
    let (mut r, mut c, mut d) = next!(usize, usize, u8);
    let mut answer = 0_usize;
    let mut step_count = 0;
    let mut is_work = false;
    let rule_a: Vec<Vec<_>> = (0..h)
        .map(|_| next!().bytes().map(|b| b - b'0').collect())
        .collect();
    let rule_b: Vec<Vec<_>> = (0..h)
        .map(|_| next!().bytes().map(|b| b - b'0').collect())
        .collect();

    let mut room = vec![vec![Dust; w]; h];

    while r < h && c < w {
        match &mut room[r][c] {
            Dust => {
                println!("Dust {step_count}");
                d = (d + rule_a[r][c]) % 4;
                room[r][c] = Clean(0);
                is_work = true;
            },
            Clean(direction) => {
                println!("Clean {step_count}");
                let i = 1 << d;
                d = (d + rule_b[r][c]) % 4;

                if *direction & i == 0 {
                    *direction |= i;
                } else {
                    break;
                }

                if is_work {
                    answer += step_count;
                    step_count = 0;
                    is_work = false;
                }
            },
        }

        match d {
            0 => r -= 1,
            1 => c += 1,
            2 => r += 1,
            3 => c -= 1,
            _ => unreachable!(),
        }

        step_count += 1;
    }

    if is_work {
        answer += step_count;
    }

    print!("{answer}");
}
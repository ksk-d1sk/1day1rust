use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut cbox = Vec::new();
    let mut visit = Vec::new();
    let mut answer = 1_u8;

    for i in 1..=n {
        cbox.push(next!().as_bytes());
        visit.push(vec![true; i]);
    }

    for i in 0..n-1 {
        for j in 0..=i {
            if
                cbox[i][j] == b'R' && cbox[i + 1][j] == b'R' && cbox[i + 1][j + 1] == b'R' &&
                visit[i][j] && visit[i + 1][j] && visit[i + 1][j + 1]
            {
                visit[i][j] = false;
                visit[i + 1][j] = false;
                visit[i + 1][j + 1] = false;
            }
        }
    }

    for i in (1..n).rev() {
        for j in 1..=(i - 1) {
            if
                cbox[i][j] == b'B' && cbox[i - 1][j] == b'B' && cbox[i - 1][j - 1] == b'B' &&
                visit[i][j] && visit[i - 1][j] && visit[i - 1][j - 1]
            {
                visit[i][j] = false;
                visit[i - 1][j] = false;
                visit[i - 1][j - 1] = false;
            }
        }
    }

    for v in visit {
        for b in v {
            if b {
                answer = 0;
            }
        }
    }

    print!("{answer}");
}
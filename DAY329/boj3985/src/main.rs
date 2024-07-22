use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let l = next!(usize);
    let n = next!(u16);
    let mut visit = vec![true; l + 1];
    let mut expected_max = (0, 0);
    let mut real_max = (0, 0);

    for i in 1..=n {
        let (p, k) = next!(usize, usize);
        let len = k - p + 1;
        let mut cnt = 0;

        if len > expected_max.1 {
            expected_max = (i, len);
        }

        for i in p..=k {
            if visit[i] {
                cnt += 1;
                visit[i] = false;
            }
        }

        if cnt > real_max.1 {
            real_max = (i, cnt);
        }
    }

    print!("{}\n{}", expected_max.0, real_max.0);
}
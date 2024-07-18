use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, u8);
    let mut house = Vec::new();
    let mut chicken = Vec::new();

    for i in 0..n {
        for j in 0..n {
            let x = next!();
            if x == "1" {
                house.push((i, j));
            } else if x == "2" {
                chicken.push((i, j));
            }
        }
    }

    let answer = backtracking(m, 0, 0, &house, &chicken, &mut vec![false; chicken.len()]);

    print!("{answer}");
}

fn backtracking(m: u8, i: usize, depth: u8, house: &[(usize, usize)], chicken: &[(usize, usize)], active: &mut [bool]) -> usize {
    if depth == m {
        let mut ret = 0;
        for &(hx, hy) in house {
            let mut diff = usize::MAX;
            for c in (0..chicken.len()).filter(|&c| active[c]) {
                let (cx, cy) = chicken[c];
                diff = diff.min(hx.abs_diff(cx) + hy.abs_diff(cy));
            }

            ret += diff;
        }

        ret
    } else {
        let mut ret = usize::MAX;
        for j in i..chicken.len() {
            active[j] = true;
            ret = backtracking(m, j + 1, depth + 1, house, chicken, active).min(ret);
            active[j] = false;
        }

        ret
    }
}
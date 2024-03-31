use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = usize::MAX;
    let (n, m, k) = next!(usize, usize, usize);
    let mut sum_table: Vec<Vec<_>> = Vec::from_iter((0..n).map(|i| {
        next!()
            .bytes()
            .enumerate()
            .map(|(j, c)| ((i & 1 == 1) ^ (j & 1 == 1) ^ (c == b'W')) as usize)
            .collect()
    }));

    for i in 0..n {
        for j in 1..m {
            sum_table[i][j] += sum_table[i][j - 1];
        }
    }

    for i in 1..n {
        for j in 0..m {
            sum_table[i][j] += sum_table[i - 1][j];
        }
    }

    for i in (k - 1)..n {
        for j in (k - 1)..m {
            let s1 = sum_table[i][j];
            let s2 = sum_table.get(i - k).and_then(|v| v.get(j - k)).unwrap_or(&0);
            let s3 = sum_table.get(i).and_then(|v| v.get(j - k)).unwrap_or(&0);
            let s4 = sum_table.get(i - k).and_then(|v| v.get(j)).unwrap_or(&0);
            let s5 = s1 + s2 - s3 - s4;

            answer = answer.min(s5.min(k * k - s5));
        }
    }

    print!("{answer}");
}
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

    let s = next!();
    let q = next!(u32);
    let mut prefix_sum = vec![[0; 26]];

    for (i, j) in s.bytes().map(|i| (i - b'a') as usize).zip(0..) {
        if j == 0 {
            prefix_sum[j][i] += 1;
        } else {
            let mut temp = prefix_sum[j - 1];
            temp[i] += 1;
            prefix_sum.push(temp);
        }
    }

    for (a, l, r) in (0..q).map(|_| next!(char, usize, usize)) {
        let a = (a as u8 - b'a') as usize;
        let _ = if l == 0 {
            writeln!(output, "{}", prefix_sum[r][a])
        } else {
            writeln!(output, "{}", prefix_sum[r][a] - prefix_sum[l - 1][a])
        };
    }

    print!("{output}");
}
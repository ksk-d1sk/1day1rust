use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let arr = Vec::from_iter((0..n).map(|_| next!(u16)));
    let mut dp1 = vec![1; n];
    let mut dp2 = vec![1; n];

    for i in 1..n {
        for j in 0..i {
            if arr[i] > arr[j] {
                dp1[i] = dp1[i].max(dp1[j] + 1);
            }
        }
    }

    for i in 1..n {
        for j in 0..i {
            let i = n - i - 1;
            let j = n - j - 1;
            if arr[i] > arr[j] {
                dp2[i] = dp2[i].max(dp2[j] + 1);
            }
        }
    }

    print!("{}", dp1.into_iter().zip(dp2).fold(0, |max, (a, b)| max.max(a + b - 1)));
}
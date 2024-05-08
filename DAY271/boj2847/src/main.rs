use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut answer = 0;
    let mut arr = Vec::from_iter((0..n).map(|_| next!(u32)));

    for i in (0..n-1).rev() {
        let t = (arr[i] + 1).saturating_sub(arr[i + 1]);
        answer += t;
        arr[i] -= t;
    }

    print!("{answer}");
}
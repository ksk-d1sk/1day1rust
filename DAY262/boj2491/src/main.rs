use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let arr = Vec::from_iter((0..n).map(|_| next!(u8)));
    let mut answer = 1;
    let mut combo = 1;

    for i in 1..n {
        if arr[i] >= arr[i - 1] {
            combo += 1;
        } else {
            answer = answer.max(combo);
            combo = 1;
        }
    }

    answer = answer.max(combo);
    combo = 1;

    for i in 1..n {
        if arr[i] <= arr[i - 1] {
            combo += 1;
        } else {
            answer = answer.max(combo);
            combo = 1;
        }
    }

    answer = answer.max(combo);

    print!("{answer}");
}
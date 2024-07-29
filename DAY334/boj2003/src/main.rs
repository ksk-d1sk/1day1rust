use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, u64);
    let v = Vec::from_iter((0..n).map(|_| next!(u64)));
    let mut answer = 0;
    let mut front = 0;
    let mut back = 0;
    let mut sum = v[0];

    while front < n && back < n {
        match sum.cmp(&m) {
            Greater => {
                sum -= v[front];
                front += 1;
            }
            Equal => {
                answer += 1;
                sum -= v[front];
                front += 1;
            }
            Less => {
                back += 1;

                if back < n {
                    sum += v[back];
                }
            }
        }
    }

    print!("{answer}");
}
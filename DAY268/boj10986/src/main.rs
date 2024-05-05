use std::collections::HashMap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(u32, u64);
    let mut answer = 0;
    let mut sum = 0;
    let mut map = HashMap::new();

    for a in (0..n).map(|_| next!(u64)) {
        sum += a;
        let c = sum % m;
        map.entry(c).and_modify(|cnt| *cnt += 1).or_insert(1_u64);
        if c == 0 {
            answer += 1;
        }
    }

    for &val in map.values() {
        answer += val * (val - 1) / 2;
    }

    print!("{answer}");
}
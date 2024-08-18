use std::collections::HashMap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let v = Vec::from_iter((0..n).map(|_| next!(i32, i32, i32, i32)));
    let mut map = HashMap::new();
    let mut answer = 0;

    for i in 0..n {
        for j in 0..n {
            map.entry(v[i].0 + v[j].1).and_modify(|e| *e += 1).or_insert(1_usize);
        }
    }

    for i in 0..n {
        for j in 0..n {
            if let Some(x) = map.get(&-(v[i].2 + v[j].3)) {
                answer += x;
            }
        }
    }

    print!("{answer}");
}
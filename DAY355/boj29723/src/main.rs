use std::collections::HashMap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m, k) = next!(usize, usize, usize);
    let mut map: HashMap<_, _> = HashMap::from_iter((0..n).map(|_| (next!(), next!(u32))));
    let mut score = 0;

    for name in (0..k).map(|_| next!()) {
        score += map[name];
        map.remove(name);
    }

    let mut min = score;
    let mut max = score;

    let mut v = Vec::from_iter(map.values());

    v.sort_unstable();

    for i in 0..(m - k) {
        let j = v.len() - i - 1;
        min += v[i];
        max += v[j];
    }

    print!("{min} {max}");
}
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = 0;
    let (n, k, c, r) = get!(usize, usize, i64, usize);
    let base: Vec<_> = (0..k).map(|_| get!(i64)).collect();
    let s: Vec<_> = (0..k).map(|_| get!(i64)).collect();
    let p: Vec<_> = (0..k).map(|_| get!(usize)).collect();
    let mut skill = vec![0; k];
    let mut fatigue = 0;
    let mut combo = 0;

    for i in (0..n).map(|_| get!(usize)) {
        if i != 0 {
            let i = i - 1;
            fatigue += p[i];

            if fatigue > 100 {
                answer = -1;
                break;
            }

            answer += base[i] * (100 + combo * c) * (100 + skill[i] * s[i]) / 10000;
            skill[i] += 1;
            combo += 1;
        } else {
            combo = 0;
            fatigue = fatigue.saturating_sub(r);
        }
    }

    print!("{answer}");
}

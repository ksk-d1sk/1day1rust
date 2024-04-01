use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($( tokens.next().unwrap().parse::<$t>().unwrap() ),+) };
    }

    let n = next!(u16);
    let mut answer = 0;

    for (a, b, c) in (0..n).map(|_| next!(u16, u16, u16)) {
        let (m1, m2) = if a > b { (a, b) } else { (b, a) };
        let (m2, m3) = if m2 > c { (m2, c) } else { (c, m2) };
        let (m1, m2) = if m1 > m2 { (m1, m2) } else { (m2, m1) };

        let reward = if m1 == m3 {
            m1 * 1000 + 10000
        } else if m1 == m2 {
            m1 * 100 + 1000
        } else if m2 == m3 {
            m2 * 100 + 1000
        } else {
            m1 * 100
        };

        answer = answer.max(reward);
    }

    print!("{answer}");
}
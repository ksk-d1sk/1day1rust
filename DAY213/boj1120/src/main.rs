use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let a = next!().as_bytes();
    let b = next!().as_bytes();
    let mut answer = u8::MAX;

    for i in 0..=(b.len() - a.len()) {
        let mut cnt = 0;
        for j in 0..a.len() {
            if a[j] != b[i + j] {
                cnt += 1;
            }
        }
        
        answer = answer.min(cnt);
    }

    print!("{answer}");
}
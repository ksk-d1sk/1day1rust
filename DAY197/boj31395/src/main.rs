use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let v: Vec<_> = (0..n).map(|_| next!(u32)).collect();
    let mut answer = 0_usize;
    let mut count = 1;

    for i in 0..n {
        if v.get(i + 1).is_some() && v[i] < v[i + 1] {
            count += 1;
        } else {
            answer += count * (count + 1) / 2;
            count = 1;
        }
    }

    print!("{answer}");
}
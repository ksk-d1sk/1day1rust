use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let v: Vec<_> = (0..n).map(|_| next!(i32)).collect();
    let mut max = v[0];
    let mut temp = v[0];

    for i in 1..n {
        if temp + v[i] > v[i] {
            temp += v[i];
        } else {
            temp = v[i];
        }

        max = max.max(temp);
    }

    print!("{max}");
}
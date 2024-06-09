use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = next!(usize, u16);
    let mut v = Vec::from_iter((0..n).map(|_| next!(u16, u32, u32, u32)));

    v.sort_unstable_by(|a, b| {
        if a.1 != b.1 {
            b.1.cmp(&a.1)
        } else if a.2 != b.2 {
            b.2.cmp(&a.2)
        } else {
            b.3.cmp(&a.3)
        }
    });

    let mut answer = 0;
    let mut temp = (u32::MAX, u32::MAX, u32::MAX);

    for i in 0..n {
        if temp != (v[i].1, v[i].2, v[i].3) {
            answer = i + 1;
        }

        if k == v[i].0 {
            break;
        }

        temp = (v[i].1, v[i].2, v[i].3);
    }

    print!("{answer}");
}
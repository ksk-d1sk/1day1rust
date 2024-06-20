use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };   
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut v = Vec::from_iter((0..n).map(|_| next!(String, u8, u8, u16)));

    v.sort_unstable_by(|a, b| {
        if a.3 != b.3 {
            a.3.cmp(&b.3)
        } else if a.2 != b.2 {
            a.2.cmp(&b.2)
        } else {
            a.1.cmp(&b.1)
        }
    });

    print!(
        "{}\n{}",
        v[n - 1].0,
        v[0].0
    );
}
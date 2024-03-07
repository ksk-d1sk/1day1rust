use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);
    let mut v: Vec<_> = (0..n)
        .map(|_| (next!().as_bytes(), next!(u32), next!(usize)))
        .collect();

    v.sort_unstable_by_key(|k| k.1);

    for (bytes, _, i) in v {
        output.push(match bytes[i - 1] {
            b@b'a'..=b'z' => (b - b'a' + b'A') as char,
            b => b as char,
        });
    }

    print!("{output}");
}
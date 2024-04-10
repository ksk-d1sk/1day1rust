use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
        () => { tokens.next().unwrap() };
    }

    let n = next!(usize);
    let mut v: Vec<_> = (0..n).map(|_| next!(u16)).collect();
    let mut q = 0;

    for (i, t) in (0..n).zip(1..) {
        if v[i] != t {
            q += 1;
            for j in i..n {
                if v[j] == t {
                    reverse(&mut v[i..=j]);
                    let _ = writeln!(output, "{} {}", i + 1, j + 1);
                    break;
                }
            }
        }
    }

    print!("{}\n{}", q, output);
}

fn reverse<T: Copy>(arr: &mut [T]) {
    let mut i = 0;
    let mut j = arr.len() - 1;
    while i < j {
        (arr[i], arr[j]) = (arr[j], arr[i]);
        i += 1;
        j -= 1;
    }
}
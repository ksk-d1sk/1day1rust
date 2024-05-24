use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let drm = next!();
    let l = drm.len();
    let left: Vec<_> = drm[0..l/2].bytes().map(|b| (b - b'A') as u32).collect();
    let right: Vec<_> = drm[l/2..].bytes().map(|b| (b - b'A') as u32).collect();
    let left_sum = left.iter().sum::<u32>();
    let right_sum = right.iter().sum::<u32>();

    for (a, b) in left.iter().zip(right) {
        let ch = (((a + b + left_sum + right_sum) % 26) as u8 + b'A') as char;
        output.push(ch);
    }

    print!("{output}");
}
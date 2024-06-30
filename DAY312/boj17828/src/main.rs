use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, mut x) = next!(u32, u32);

    if x < n || n * 26 < x {
        output.push('!');
    } else {
        x -= n;

        let i = x / 25;
        let j = (x % 25) as u8;

        if n > i {
            for _ in 0..(n - i - 1) {
                output.push('A');
            }

            output.push((j + b'A') as char);
        }

        for _ in 0..i {
            output.push('Z');
        }
    }

    print!("{output}");
}
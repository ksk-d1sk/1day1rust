use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let s = next!();
    let a = format!("0000{s}");
    let a = a.as_bytes();
    let b = format!("{s}0000");
    let b = b.as_bytes();
    let mut u = false;
    let mut stack = Vec::new();

    for i in (0..a.len()).rev() {
        if a[i] == b'1' && b[i] == b'1' {
            if u {
                stack.push('1');
            } else {
                stack.push('0');
            }
            u = true;
        } else if a[i] == b'0' && b[i] == b'0' {
            if u {
                stack.push('1');
            } else {
                stack.push('0');
            }
            u = false;
        } else {
            if u {
                stack.push('0');
            } else {
                stack.push('1');
                u = false;
            }
        }
    }

    if u {
        stack.push('1');
    }

    while let Some(ch) = stack.pop() {
        output.push(ch);
    }

    let mut output = output.trim_start_matches('0');

    if output.is_empty() {
        output = "0";
    }
    print!("{output}");
}
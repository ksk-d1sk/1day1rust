use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    for password in (0..).map(|_| next!()).take_while(|pwd| *pwd != "end") {
        let mut final_check = true;
        let mut moeum_exist_check = false;
        let mut moeum_continuous_count = 0;
        let mut jaeum_continuous_count = 0;
        let mut last_character = b' ';

        for b in password.bytes() {
            if !matches!(b, b'e' | b'o') && b == last_character {
                final_check = false;
                break;
            } else {
                last_character = b;
            }

            if matches!(b, b'a' | b'e' | b'i' | b'o' | b'u') {
                moeum_continuous_count += 1;
                jaeum_continuous_count = 0;
                moeum_exist_check = true;

                if moeum_continuous_count == 3 {
                    final_check = false;
                    break;
                }
            } else {
                jaeum_continuous_count += 1;
                moeum_continuous_count = 0;

                if jaeum_continuous_count == 3 {
                    final_check = false;
                    break;
                }
            }
        }

        final_check &= moeum_exist_check;

        let _ = if final_check {
            writeln!(output, "<{password}> is acceptable.")
        } else {
            writeln!(output, "<{password}> is not acceptable.")
        };
    }

    print!("{output}");
}
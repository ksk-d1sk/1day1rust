use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut cnt = 0;
    let mut check = true;

    for b in next!().bytes() {
        if b == b'X' {
            cnt += 1;
        } else {
            if cnt != 0 {
                if cnt & 1 == 0 {
                    for _ in 0..(cnt / 4) {
                        output.push_str("AAAA");
                    }

                    if cnt % 4 == 2 {
                        output.push_str("BB");
                    }
                } else {
                    check = false;
                    cnt = 0;
                    break;
                }
            }

            cnt = 0;
            output.push('.');
        }
    }

    if cnt & 1 == 0 {
        for _ in 0..(cnt / 4) {
            output.push_str("AAAA");
        }

        if cnt % 4 == 2 {
            output.push_str("BB");
        }
    } else {
        check = false;
    }

    if check {
        print!("{output}");
    } else {
        print!("-1");
    }
}
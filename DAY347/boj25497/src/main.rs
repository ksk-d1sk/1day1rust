use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    next!();
    let mut answer = 0_usize;
    let mut lr = 0;
    let mut sk = 0;

    for b in next!().bytes() {
        match b {
            b'L' => lr += 1,
            b'S' => sk += 1,
            b'R' => {
                if lr == 0 {
                    break;
                }
                lr -= 1;
                answer += 1;
            }
            b'K' => {
                if sk == 0 {
                    break;
                }
                sk -= 1;
                answer += 1;
            }
            _ => answer += 1,
        }
    }

    print!("{answer}");
}
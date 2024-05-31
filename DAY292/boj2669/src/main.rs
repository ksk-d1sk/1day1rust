use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut count = 0;
    let arr = [
        next!(u16, u16, u16, u16),
        next!(u16, u16, u16, u16),
        next!(u16, u16, u16, u16),
        next!(u16, u16, u16, u16),
    ];

    for i in 1..=100 {
        for j in 1..=100 {
            for (x1, y1, x2, y2) in arr {
                if x1 < i && y1 < j && i <= x2 && j <= y2 {
                    count += 1;
                    break;
                }
            }
        }
    }

    print!("{count}");
}
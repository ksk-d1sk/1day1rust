use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(u8);
    let korea = next!(u32);
    let math            = if t > 1 { next!(u32) } else { 0 };
    let english         = if t > 2 { next!(u32) } else { 0 };
    let tam9            = if t > 3 { next!(u32) } else { 0 };
    let second_lenguage = if t > 4 { next!(u32) } else { 0 };

    print!(
        "{}", (
            if korea > english {
                (korea - english) * 508
            } else {
                (english - korea) * 108
            }
            +
            if math > tam9 {
                (math - tam9) * 212
            } else {
                (tam9 - math) * 305
            }
            +
            second_lenguage * 707
        ) * 4763
    );
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(i64);
    let mut x = n;
    let mut i = 4052555153018976267_i64;

    while x > 0 && i > 0 {
        if x >= i {
            x -= i;
        }

        i /= 3;
    }

    if x == 0 && n != 0 {
        print!("YES");
    } else {
        print!("NO");
    }
}
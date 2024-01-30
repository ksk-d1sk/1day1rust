use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let arr = ['S', 'W', 'N', 'E'];
    let mut i = 10;

    for x in (0..10).map(|_| next!(u8)) {
        if x == 1 {
            i += 1;
        } else if x == 2 {
            i += 2;
        } else {
            i -= 1;
        }
    }

    print!("{}", arr[i % 4]);
}
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let arr = [
        (0, 0),
        (12, 1600),
        (11, 894),
        (11, 1327),
        (10, 1311),
        (9, 1004),
        (9, 1178),
        (9, 1357),
        (8, 837),
        (7, 1055),
        (6, 556),
        (6, 773),
    ];

    print!("{} {}", arr[n].0, arr[n].1);
}
use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, u32);
    let mut queue = VecDeque::new();
    let mut max_volume = 0;
    let mut volume = 0;

    for x in (0..n).map(|_| next!(u32)) {
        volume += x;
        queue.push_back(x);

        while volume > m {
            volume -= queue.pop_front().unwrap();
        }

        max_volume = max_volume.max(volume);
    }

    print!("{max_volume}");
}
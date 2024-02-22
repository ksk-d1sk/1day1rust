use std::collections::VecDeque;
use std::io::*;

const MAX: usize = 100_001;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = next!(usize, usize);
    let mut arr = [(-1, 0); MAX];
    let mut queue = VecDeque::new();

    queue.push_back((n, 0));
    arr[n] = (0, 1);

    while let Some((x, cnt)) = queue.pop_front() {
        for nx in [x - 1, x + 1, x * 2].into_iter().filter(|&nx| nx < MAX) {
            if arr[nx].1 == 0 || cnt < arr[nx].0 {
                arr[nx].0 = cnt + 1;
                arr[nx].1 += 1;
                queue.push_back((nx, cnt + 1));
            }
        }
    }

    print!("{}\n{}", arr[k].0, arr[k].1);
}
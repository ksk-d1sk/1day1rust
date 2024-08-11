use std::collections::VecDeque;
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

    let n = next!(usize);
    let mut deque = VecDeque::from_iter((0..n).map(|i| (i, next!(u8))));
    let mut answer = vec![0; n];
    let mut count = 0;

    while let Some((i, x)) = deque.pop_front() {
        count += 1;

        if x == 1 {
            answer[i] = count;
        } else {
            deque.push_back((i, x - 1));
        }
    }

    for x in answer {
        let _ = write!(output, "{x} ");
    }

    print!("{output}");
}
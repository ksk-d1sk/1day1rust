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

    let n = next!(u16);
    let mut queue: VecDeque<_> = (1..=n).collect();

    while let Some(a) = queue.pop_front() {
        let _ = write!(output, "{a} ");

        if let Some(b) = queue.pop_front() {
            queue.push_back(b);
        }
    }

    print!("{output}");
}
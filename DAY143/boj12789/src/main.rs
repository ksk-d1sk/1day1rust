use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut stack_input: VecDeque<_> = (0..n).map(|_| get!(u32)).collect();
    let mut stack_wait = Vec::with_capacity(n);
    let mut next_num = 1;
    let mut answer = "Nice";

    while let Some(&i) = stack_input.front() {
        if i == next_num {
            next_num += 1;
            stack_input.pop_front();
        } else if stack_wait.last() == Some(&next_num) {
            next_num += 1;
            stack_wait.pop();
        } else {
            if stack_wait.is_empty() || Some(&i) < stack_wait.last() {
                stack_wait.push(
                    stack_input.pop_front().unwrap()
                );
            } else {
                answer = "Sad";
                break;
            }
        }
    }

    print!("{answer}");
}

use std::collections::VecDeque;
use std::io::Read;
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut get = || input.next().unwrap();

    let mut deque = VecDeque::with_capacity(1_000_000);

    for _ in 0..get().parse().unwrap() {
        let _ = match get() {
            "1" => {
                deque.push_front(get());
                continue;
            },
            "2" => {
                deque.push_back(get());
                continue;
            },
            "3" => writeln!(output, "{}", deque.pop_front().unwrap_or("-1")),
            "4" => writeln!(output, "{}", deque.pop_back().unwrap_or("-1")),
            "5" => writeln!(output, "{}", deque.len()),
            "6" => writeln!(output, "{}", u8::from(deque.is_empty())),
            "7" => writeln!(output, "{}", deque.front().unwrap_or(&"-1")),
            "8" => writeln!(output, "{}", deque.back().unwrap_or(&"-1")),
            _ => panic!(),
        };
    }

    print!("{output}");
}
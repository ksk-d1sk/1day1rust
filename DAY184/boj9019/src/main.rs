use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = get!(usize);
    let mut queue = VecDeque::new();
    for (a, b) in (0..t).map(|_| get!(usize, usize)) {
        let mut visit = [true; 10_000];

        queue.push_back((a, String::new()));
        visit[a] = false;

        while let Some((x, buf)) = queue.pop_front() {
            for (dx, c) in [d(x), s(x), l(x), r(x)] {
                if dx == b {
                    let _ = writeln!(output, "{}{}", buf, c);
                    queue.clear();
                    break;
                }

                if visit[dx] {
                    visit[dx] = false;
                    queue.push_back((dx, format!("{}{}", buf, c)));
                }
            }
        }
    }

    print!("{output}");
}

fn d(x: usize) -> (usize, char) {
    (x * 2 % 10000, 'D')
}

fn s(x: usize) -> (usize, char) {
    (if x == 0 { 9999 } else { x - 1 }, 'S')
}

fn l(x: usize) -> (usize, char) {
    (x % 1000 * 10 + x / 1000, 'L')
}

fn r(x: usize) -> (usize, char) {
    (x / 10 + x % 10 * 1000, 'R')
}
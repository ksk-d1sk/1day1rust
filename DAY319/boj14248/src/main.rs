use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let v = Vec::from_iter((0..n).map(|_| next!(usize)));
    let mut visit = vec![true; n];
    let mut queue = VecDeque::new();
    let mut answer = 0;
    let start = next!(usize);

    queue.push_back(start - 1);
    visit[start - 1] = false;

    while let Some(x) = queue.pop_front() {
        answer += 1;

        for nx in [x + v[x], x - v[x]] {
            if nx < n && visit[nx] {
                visit[nx] = false;
                queue.push_back(nx);
            }
        }
    }

    print!("{answer}");
}
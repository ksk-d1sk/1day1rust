use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    // let input = read_to_string(std::fs::File::open("input.txt").unwrap()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let v = Vec::from_iter((0..n).map(|_| next!(usize)));
    let (a, b) = next!(usize, usize);
    let mut answer = -1;
    let mut queue = VecDeque::new();
    let mut visit = vec![true; n];

    queue.push_back((a - 1, 0));
    visit[a - 1] = false;

    while let Some((x, cnt)) = queue.pop_front() {
        if x + 1 == b {
            answer = cnt;
            break;
        }

        for nx in (1..).map(|i| x + v[x] * i).take_while(|&nx| nx < n) {
            if visit[nx] {
                visit[nx] = false;
                queue.push_back((nx, cnt + 1));
            }
        }

        for nx in (1..).map(|i| x - v[x] * i).take_while(|&nx| nx < n) {
            if visit[nx] {
                visit[nx] = false;
                queue.push_back((nx, cnt + 1));
            }
        }
    }

    print!("{answer}");
}
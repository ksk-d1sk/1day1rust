use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, u32);
    let (s, e) = next!(usize, usize);

    let mut answer = 0;
    let mut queue = VecDeque::new();
    let mut visit = vec![true; n];
    let mut adj = vec![Vec::new(); n];

    for (x, y) in (0..m).map(|_| next!(usize, usize)) {
        adj[x - 1].push(y - 1);
        adj[y - 1].push(x - 1);
    }

    queue.push_back((s - 1, 0));
    visit[s - 1] = false;

    while let Some((x, cnt)) = queue.pop_front() {
        if x + 1 == e {
            answer = cnt;
            break;
        }

        for &nx in adj[x].iter().chain([x + 1, x - 1].iter()) {
            if nx < n && visit[nx] {
                visit[nx] = false;
                queue.push_back((nx, cnt + 1));
            }
        }
    }

    print!("{answer}");
}
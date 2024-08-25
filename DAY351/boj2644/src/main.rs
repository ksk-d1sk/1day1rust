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
    let (s, e) = next!(usize, usize);
    let m = next!(u32);
    let mut edge = vec![Vec::new(); n + 1];
    let mut visit = vec![true; n + 1];
    let mut queue = VecDeque::new();
    let mut answer = -1;

    for (a, b) in (0..m).map(|_| next!(usize, usize)) {
        edge[a].push(b);
        edge[b].push(a);
    }

    queue.push_back((s, 0));
    visit[s] = false;

    while let Some((x, cnt)) = queue.pop_front() {
        if x == e {
            answer = cnt;
            break;
        }

        for &nx in edge[x].iter() {
            if visit[nx] {
                visit[nx] = false;
                queue.push_back((nx, cnt + 1));
            }
        }
    }

    print!("{answer}");
}
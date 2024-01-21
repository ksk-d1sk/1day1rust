use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = None;
    let (f, s, g, u, d) = get!(usize, usize, usize, usize, usize);
    let mut visit = vec![true; f];
    let mut queue = VecDeque::new();

    queue.push_back((s - 1, 0));
    visit[s - 1] = false;

    while let Some((x, cnt)) = queue.pop_front() {
        if x + 1 == g {
            answer = Some(cnt);
            break;
        }

        for nx in [x + u, x - d] {
            if nx < f && visit[nx] {
                visit[nx] = false;
                queue.push_back((nx, cnt + 1));
            }
        }
    }

    if let Some(count) = answer {
        print!("{count}");
    } else {
        print!("use the stairs");
    }
}
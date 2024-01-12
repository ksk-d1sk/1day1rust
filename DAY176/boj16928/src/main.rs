use std::collections::{HashMap, VecDeque};
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let mut visit = [u8::MAX; 101];
    let mut ladders = HashMap::with_capacity(n);
    let mut snakes = HashMap::with_capacity(m);
    let mut queue = VecDeque::new();

    for (x, y) in (0..n).map(|_| get!(usize, usize)) {
        ladders.insert(x, y);
    }

    for (u, v) in (0..m).map(|_| get!(usize, usize)) {
        snakes.insert(u, v);
    }

    queue.push_back((1, 0));

    while let Some((idx, cnt)) = queue.pop_front() {
        if idx == 100 {
            print!("{cnt}");
            break;
        }

        for i in 1..=6 {
            let mut nidx = idx + i;
            let ncnt = cnt + 1;

            nidx = *ladders.get(&nidx).unwrap_or(&nidx);
            nidx = *snakes.get(&nidx).unwrap_or(&nidx);

            if nidx < 101 && ncnt < visit[nidx] {
                visit[nidx] = ncnt;
                queue.push_back((nidx, ncnt));
            }
        }
    }
}
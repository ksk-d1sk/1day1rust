use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let mut graph = vec![Vec::new(); n];
    let mut answer = (0, usize::MAX);
    let mut queue = VecDeque::new();

    for (a, b) in (0..m).map(|_| get!(usize, usize)) {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    for i in 0..n {
        let mut visit = vec![usize::MAX; n];

        queue.push_back((i, 1));
        visit[i] = 0;

        while let Some((idx, cnt)) = queue.pop_front() {
            for &nidx in graph[idx].iter() {
                if cnt < visit[nidx] {
                    visit[nidx] = cnt;
                    queue.push_back((nidx, cnt + 1));
                }
            }
        }

        let kevin_bacon: usize = visit.iter().sum();
        if kevin_bacon < answer.1 {
            answer.0 = i + 1;
            answer.1 = kevin_bacon;
        }
    }

    print!("{}", answer.0);
}
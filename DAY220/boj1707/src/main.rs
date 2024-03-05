use std::collections::VecDeque;
use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.trim_end().split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(usize);
    let mut queue = VecDeque::new();

    for _ in 0..t {
        let (v, e) = next!(usize, usize);
        let mut edge = vec![Vec::new(); v + 1];
        let mut visit = vec![0; v + 1];
        let mut check = true;

        for (u, v) in (0..e).map(|_| next!(usize, usize)) {
            edge[u].push(v);
            edge[v].push(u);
        }

        for i in 1..=v {
            if visit[i] == 0 && !edge[i].is_empty() {
                visit[i] = 1;
                queue.extend(edge[i].iter().copied().map(|x| (x, 2_u32)));

                while let Some((x, depth)) = queue.pop_front() {
                    if visit[x] == 0 {
                        visit[x] = depth;
                        queue.extend(edge[x].iter().copied().map(|nx| (nx, depth + 1)));
                    } else if visit[x] & 1 != depth & 1 {
                        check = false;
                        queue.clear();
                    }
                }
            }

            if !check { break }
        }

        let _ = if check {
            writeln!(output, "YES")
        } else {
            writeln!(output, "NO")
        };
    }

    print!("{output}");
}
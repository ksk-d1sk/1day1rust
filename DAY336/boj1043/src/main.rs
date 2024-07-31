use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let mut start = Vec::from_iter({
        let t = next!(u8);
        (0..t).map(|_| next!(usize))
    });
    let mut edge = vec![Vec::new(); n + 1];
    let mut party = vec![Vec::new(); m + 1];
    let mut visit = vec![true; m + 1];
    let mut answer = m;

    for i in 1..=m {
        let t = next!(u8);
        for p in (0..t).map(|_| next!(usize)) {
            edge[p].push(i);
            party[i].push(p);
        }
    }

    while let Some(s) = start.pop() {
        for &r in edge[s].iter() {
            if visit[r] {
                visit[r] = false;
                answer -= 1;

                for &p in party[r].iter() {
                    start.push(p);
                }
            }
        }
    }

    print!("{answer}");
}
use std::collections::VecDeque;
use std::io::*;

const MAX: usize = 100_001;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = next!(usize, usize);
    let mut arr = [-1; MAX];
    let mut queue = VecDeque::from_iter(
        (0..)
            .map(|i| (n * (1 << i), i))
            .take_while(|&(x, i)| (x != 0 || i == 0) && x < MAX && {arr[x] = 0; true})
            .map(|(x, _)| (x, 0))
    );

    while let Some((i, cnt)) = queue.pop_front() {
        if i == k { break }

        for ni in [i - 1, i + 1] {
            for (x, _) in (0..)
                .map(|j| (ni * (1 << j), j))
                .take_while(|&(x, j)| (x != 0 || j == 0) && x < MAX)
            {
                if arr[x] == -1 {
                    arr[x] = cnt + 1;
                    queue.push_back((x, cnt + 1));
                }
            }
        }
    }

    print!("{}", arr[k]);
}
use std::collections::VecDeque;
use std::fmt::Write;
use std::io::*;

const MAX: usize = 100_001;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = next!(usize, usize);
    let mut arr = vec![(-1, None); MAX];
    let mut queue = VecDeque::new();

    queue.push_back((n, 0));
    arr[n].0 = 0;

    while let Some((x, cnt)) = queue.pop_front() {
        if x == k { break }

        for nx in [x - 1, x + 1, x * 2] {
            if nx < MAX && arr[nx].0 == -1 {
                arr[nx] = (cnt + 1, Some(x));
                queue.push_back((nx, cnt + 1));
            }
        }
    }

    let mut answer = Vec::new();
    let mut i = k;
    
    while let Some(ni) = arr[i].1 {
        answer.push(ni);
        i = ni;
    }

    let _ = writeln!(output, "{}", arr[k].0);
    for x in answer.iter().rev() {
        let _ = write!(output, "{x} ");
    }
    let _ = write!(output, "{k}");


    print!("{output}");
}
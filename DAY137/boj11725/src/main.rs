use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut link = vec![Vec::new(); n + 1];
    let mut stack = Vec::with_capacity(n);
    let mut answer = vec![0; n + 1];

    for (l, r) in (1..n).map(|_| get!(usize, usize)) {
        link[l].push(r);
        link[r].push(l);
    }

    stack.push(1);

    while let Some(i) = stack.pop() {
        for &j in link[i].iter() {
            if !link[j].is_empty() {
                answer[j] = i;
            }
        }

        stack.append(&mut link[i]);
    }

    for a in &answer[2..] {
        let _ = writeln!(output, "{}", a);
    }

    print!("{output}");
}
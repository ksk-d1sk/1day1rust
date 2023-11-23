use std::io;

fn main() {
    let input = 
        // include_str!("input.txt");
        io::read_to_string(io::stdin()).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let mut ans = vec![String::with_capacity(m); n];

    for i in 0..n {
        if let Some(line) = get!() {
            for c in line.chars().rev() {
                ans[i].push(c);
            }
        }
    }

    print!("{}", ans.join("\n").trim());
}

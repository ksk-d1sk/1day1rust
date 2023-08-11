use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $t:ty ) => { input.next().unwrap().parse::<$t>().unwrap() };
    }

    let n = get!(usize);
    let mut dp = Vec::with_capacity(n + 1);
    let mut point = Vec::with_capacity(n + 1);

    point.push(0);
    
    for i in input.flat_map(str::parse) {
        point.push(i);
    }

    dp.push(0);
    dp.push(point[1]);
    if n > 1 { dp.push(point[1] + point[2]) }

    for i in 3..=n {
        dp.push(dp[i - 2].max(dp[i - 3] + point[i - 1]) + point[i]);
    }

    print!("{}", dp.pop().unwrap());
}
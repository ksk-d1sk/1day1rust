// use std::io;

fn main() {
    // let buf = io::read_to_string(io::stdin()).unwrap();
    // let mut input = buf.split_ascii_whitespace();

    let mut dp = [
        vec![vec![1_u32]],
        vec![vec![2], vec![1, 1]],
    ];

    for i in 2..3 {
        for his in dp.get_mut(i & 1).unwrap() {
            his.push(2);
        }

        for his in dp[i & 1 ^ 1].iter() {
            if *his.last().unwrap() != 1 {
                dp[i & 1].push([his.clone(), vec![1]].concat());
            }
        }
    }

    // 내일 마저 푸는걸로..
    print!("{:?}", dp);
}

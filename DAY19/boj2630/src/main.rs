// 색종이 만들기

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut paper: Vec<Vec<_>> = Vec::with_capacity(n);

    for line in input {
        paper.push(line.split_ascii_whitespace().flat_map(str::parse).collect());
    }

    let mut answer = (0, 0);
    solve((0, 0), n, &paper, &mut answer);
    print!("{}\n{}", answer.0, answer.1);
}

fn solve(point: (usize, usize), n: usize, paper: &Vec<Vec<u8>>, answer: &mut (u16, u16)) {
    let mut sum = 0;
    for i in point.0..(point.0 + n) {
        for j in point.1..(point.1 + n) {
            if paper[i][j] == 1 {
                sum += 1;
            }
        }
    }

    if sum == 0 {
        answer.0 += 1;
    } else if sum == (n * n) as u16 {
        answer.1 += 1;
    } else {
        solve(point, n / 2, paper, answer);
        solve((point.0 + n / 2, point.1), n / 2, paper, answer);
        solve((point.0, point.1 + n / 2), n / 2, paper, answer);
        solve((point.0 + n / 2, point.1 + n / 2), n / 2, paper, answer);
    }
}
// 다음에 마저 풀어야지

use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut edge = vec![Vec::with_capacity(n); n];
    let mut visit = vec![vec![false; n]; n];

    for i in 0..n {
        for j in 0..n {
            if next!() == "1" {
                edge[i].push(j);
            }
        }
    }

    for i in 0..n {
        if !visit[i][i] {
            let mut nodes = Vec::with_capacity(n);
            nodes.push(i);
            dfs(i, &mut nodes, &edge, &mut visit);
        }
    }

    for i in 0..n {
        for j in 0..n {
            if visit[i][j] {
                output.push('1');
            } else {
                output.push('0');
            }
            output.push(' ');
        }
        output.push('\n');
    }

    print!("{output}");
}

fn dfs(i: usize, nodes: &mut Vec<usize>, edge: &[Vec<usize>], visit: &mut [Vec<bool>]) {
    for &j in edge[i].iter() {
        if !visit[j][j] {
            for &previous_node in nodes.iter() {
                visit[previous_node][j] = true;
            }
    
            nodes.push(j);
            dfs(j, nodes, edge, visit);
            nodes.pop();
        }
    }
}
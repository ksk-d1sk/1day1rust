use std::collections::VecDeque;
use std::io::*;

struct PrefixTable {
    n: usize,
    m: usize,
    table: Vec<Vec<u32>>,
}

impl PrefixTable {
    fn new(n: usize, m: usize, v: &[Vec<u32>]) -> Self {
        let mut table = Vec::new();

        for i in 0..n {
            let mut tmp = vec![v[i][0]];
            for j in 1..m {
                tmp.push(tmp[j - 1] + v[i][j]);
            }

            table.push(tmp);
        }

        for j in 0..m {
            for i in 1..n {
                table[i][j] += table[i - 1][j];
            }
        }

        Self{n, m, table}
    }

    fn get(&self, i: usize, j: usize, h: usize, w: usize) -> Option<u32> {
        if i + h - 1 < self.n && j + w - 1 < self.m {
            let mut ret = self.table[i + h - 1][j + w - 1];

            if i != 0 && j != 0 {
                ret += self.table[i - 1][j - 1];
            }

            if i != 0 {
                ret -= self.table[i - 1][j + w - 1];
            }
    
            if j != 0 {
                ret -= self.table[i + h - 1][j - 1];
            }

            Some(ret)
        } else {
            None
        }
    }
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let v: Vec<Vec<_>> = (0..n).map(|_| (0..m).map(|_| next!(u32)).collect()).collect();
    let table = PrefixTable::new(n, m, &v);
    let (h, w) = next!(usize, usize);
    let (sr, sc) = next!(usize, usize);
    let e = next!(usize, usize);
    let mut visit = vec![vec![true; m]; n];
    let mut queue = VecDeque::new();
    let mut answer = -1;

    queue.push_back((sr - 1, sc - 1, 0));
    visit[sr - 1][sc - 1] = false;

    while let Some((x, y, cnt)) = queue.pop_front() {
        if (x + 1, y + 1) == e {
            answer = cnt;
            break;
        }

        for (nx, ny) in [
            (x + 1, y), (x, y + 1),
            (x - 1, y), (x, y - 1),
        ] {
            if nx < n && ny < m && visit[nx][ny] && table.get(nx, ny, h, w) == Some(0) {
                visit[nx][ny] = false;
                queue.push_back((nx, ny, cnt + 1));
            }
        }
    }

    print!("{answer}");
}
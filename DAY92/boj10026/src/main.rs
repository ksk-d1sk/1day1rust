use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut count1 = 0;
    let mut count2 = 0;
    let mut picture: Vec<Vec<char>> = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|e| e.chars().collect())
        .collect();
    let mut availability = init_availability_node(n);

    /* 적록색약이 아닌 사람의 dfs */
    for i in 0..n {
        for j in 0..n {
            if availability[i][j] {
                count1 += 1;
                dfs(&picture, &mut availability, i, j);
            }
        }
    }

    /* 그림의 G를 R로 변경 */
    for i in 0..n {
        for j in 0..n {
            if picture[i][j] == 'G' {
                picture[i][j] = 'R';
            }
        }
    }

    /* 방문 노드 초기화 */
    availability = init_availability_node(n);

    /* 적록색약인 사람의 dfs */
    for i in 0..n {
        for j in 0..n {
            if availability[i][j] {
                count2 += 1;
                dfs(&picture, &mut availability, i, j);
            }
        }
    }

    print!("{} {}", count1, count2);
}

fn dfs(
    grid: &Vec<Vec<char>>,
    availability: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize
) {
    availability[i][j] = false;

    for (option_nx, option_ny) in [
        (i.checked_add(1), j.checked_add(0)),
        (i.checked_sub(1), j.checked_add(0)),
        (i.checked_add(0), j.checked_add(1)),
        (i.checked_add(0), j.checked_sub(1)),
    ] {
        if let (Some(nx), Some(ny)) = (option_nx, option_ny) {
            if grid.get(nx).and_then(|e| e.get(ny)) == Some(&grid[i][j]) && availability[nx][ny] {
                dfs(grid, availability, nx, ny);
            }
        }
    }
}

fn init_availability_node(size: usize) -> Vec<Vec<bool>> {
    (0..size)
        .map(|_| (0..size).map(|_| true).collect())
        .collect()
}
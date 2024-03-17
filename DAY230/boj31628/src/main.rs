use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let grid: Vec<Vec<_>> = (0..10).map(|_| (0..10).map(|_| next!()).collect()).collect();
    let mut check = false;

    for i in 0..10 {
        let first = grid[i][0];
        let mut is_all_same = true;

        for j in 1..10 {
            if first != grid[i][j] {
                is_all_same = false;
            }
        }

        check |= is_all_same;
    }

    for i in 0..10 {
        let first = grid[0][i];
        let mut is_all_same = true;

        for j in 1..10 {
            if first != grid[j][i] {
                is_all_same = false;
            }
        }

        check |= is_all_same;
    }

    if check {
        print!("1");
    } else {
        print!("0");
    }
}
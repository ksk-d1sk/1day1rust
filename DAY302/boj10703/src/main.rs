use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (r, s) = next!(usize, usize);
    let mut answer_grid = vec![vec![b'.'; s]; r];
    let mut original_grid = vec![vec![b'.'; s]; r];
    let mut meteor_fragment = Vec::new();
    let mut min_height = usize::MAX;

    for i in 0..r {
        for (b, j) in next!().bytes().zip(0..) {
            match b {
                b'.' => {}
                b'X' => {
                    original_grid[i][j] = b'X';
                    meteor_fragment.push((i, j));
                }
                b'#' => {
                    original_grid[i][j] = b'#';
                    answer_grid[i][j] = b'#';
                }
                _ => unreachable!()
            }
        }
    }

    for j in 0..s {
        let mut height = 0;
        let mut check = false;

        for i in 0..r {
            if original_grid[i][j] == b'X' {
                check = true;
                height = 0;
            } else if original_grid[i][j] == b'.' {
                height += 1;
            } else {
                break;
            }
        }

        if check {
            min_height = min_height.min(height);
        }
    }

    for (i, j) in meteor_fragment {
        answer_grid[i + min_height][j] = b'X';
    }

    for v in answer_grid {
        for ch in v {
            output.push(ch as char);
        }
        output.push('\n');
    }

    print!("{output}");
}
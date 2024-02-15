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
    let mut matrix = vec![Vec::with_capacity(n); n];

    for i in 0..n {
        for _ in 0..n {
            matrix[i].push(next!() == "1");
        }
    }

    for m in 0..n {
        for i in 0..n {
            for j in 0..n {
                if matrix[i][m] && matrix[m][j] {
                    matrix[i][j] = true;
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if matrix[i][j] {
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
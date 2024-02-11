use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);
    let mut grid = vec![vec!['.'; 20]; 10];

    for s in (0..n).map(|_| next!()) {
        let r = s[0..1].parse::<char>().unwrap() as usize - 65;
        let c = s[1..].parse::<usize>().unwrap() - 1;

        grid[r][c] = 'o';
    }

    print!(
        "{}",
        grid.into_iter()
            .map(|v| v.into_iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
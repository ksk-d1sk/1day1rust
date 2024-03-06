use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(i32);
    let mut button = [true; 10];
    let m = next!(u8);
    let mut answer = (100 - n).abs();
    
    for x in (0..m).map(|_| next!(usize)) {
        button[x] = false;
    }

    let button: Vec<i32> = button
        .into_iter()
        .zip(0..)
        .filter_map(|(x, i)| x.then_some(i))
        .collect();

    answer = dfs(n, 0, &button, 0).min(answer);

    print!("{answer}");
}

fn dfs(n: i32, x: i32, button: &[i32], depth: i32) -> i32 {
    if depth == 6 {
        (n - x).abs() + depth
    } else {
        let mut min = if depth == 0 {
            i32::MAX
        } else { 
            (n - x).abs() + depth
        };

        for i in 0..button.len() {
            min = dfs(n, x * 10 + button[i], button, depth + 1).min(min);
        }

        min
    }
}
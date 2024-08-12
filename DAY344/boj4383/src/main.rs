use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    while let Some(n) = tokens.next() {
        let n = n.parse().unwrap();
        let v = Vec::from_iter((0..n).map(|_| next!(i64)));
        let mut check = vec![false; n - 1];

        for window in v.windows(2) {
            let x = (window[0] - window[1]).abs() as usize - 1;
            if x < n - 1 {
                check[x] = true;
            }
        }

        if check.into_iter().all(|b| b) {
            output.push_str("Jolly\n");
        } else {
            output.push_str("Not jolly\n");
        }
    }

    print!("{output}");
}
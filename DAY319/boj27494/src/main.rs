use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u32);
    let mut answer = 0;

    for i in 2023..=n {
        if find_2023(i) {
            answer += 1;
        }
    }

    print!("{answer}");
}

fn find_2023(mut n: u32) -> bool {
    let arr = [3, 2, 0, 2];
    let mut i = 0;

    while i != 4 && n > 0 {
        if arr[i] == n % 10 {
            i += 1;
        }
        n /= 10;
    }

    i == 4
}
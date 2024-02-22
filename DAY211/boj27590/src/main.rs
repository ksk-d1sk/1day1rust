use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let (ds, ys) = next!(usize, usize);
    let (dm, ym) = next!(usize, usize);
    let mut arr = [0_u8; 5001];

    for i in (1..).map(|i| ys * i - ds).take_while(|&i| i < 5001) {
        arr[i] += 1;
    }

    for i in (1..).map(|i| ym * i - dm).take_while(|&i| i < 5001) {
        arr[i] += 1;
    }

    for i in 1.. {
        if arr[i] == 2 {
            print!("{i}");
            return;
        }
    }
}
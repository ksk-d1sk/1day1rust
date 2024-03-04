use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (x, y) = next!(i32, i32);
    let mut dif = y - x;
    let mut answer = 0;
    
    'a: for i in 1.. {
        for _ in 0..2 {
            if dif <= 0 { break 'a; }
            answer += 1;
            dif -= i;
        }
    }
    
    print!("{answer}");
}
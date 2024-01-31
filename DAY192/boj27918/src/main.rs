use std::cell::Cell;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u32);
    let d_count = Cell::new(0_i32);
    let p_count = Cell::new(0_i32);

    for s in (0..n).map(|_| next!()).take_while(|_| (d_count.get() - p_count.get()).abs() < 2) {
        let temp;
        if s == "D" {
            temp = d_count.get();
            d_count.set(temp + 1);
        } else {
            temp = p_count.get();
            p_count.set(temp + 1);
        }
    }

    print!("{}:{}", d_count.get(), p_count.get());
}
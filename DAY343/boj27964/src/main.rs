use std::collections::HashSet;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut set = HashSet::new();

    for cheese in (0..n).map(|_| next!()) {
        let l = cheese.len();
        if cheese.get(l-6..l) == Some("Cheese") {
            set.insert(cheese);
        }
    }

    if set.len() >= 4 {
        print!("yummy");
    } else {
        print!("sad");
    }
}
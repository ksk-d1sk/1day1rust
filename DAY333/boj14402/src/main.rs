use std::collections::HashMap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let q = next!(u32);
    let mut map = HashMap::new();
    let mut answer = 0;

    for (name, h) in (0..q).map(|_| (next!(), next!())) {
        map.entry(name).or_insert(0_u32);

        if h == "+"{
            map.entry(name).and_modify(|count| *count += 1);
        } else {
            map.entry(name).and_modify(|count| {
                if *count == 0 {
                    answer += 1;
                } else {
                    *count -= 1;
                }
            });
        }
    }

    for c in map.values() {
        answer += c;
    }

    print!("{answer}");
}
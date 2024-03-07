use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $t:ty ) => {
            tokens.next().unwrap().parse::<$t>().unwrap()
        };
    }

    let n = next!(u8);
    let check = (0..3)
        .map(|_| {
            (0..n)
                .map(|_| next!(u8))
                .collect::<Vec<_>>()
                .into_iter()
                .any(|x| x == 7)
        })
        .all(|b| b);

    if check {
        print!("777");
    } else {
        print!("0");
    }
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let tokens = input.split_ascii_whitespace().flat_map(str::parse::<u8>);

    let mut count_1 = 0;
    let mut count_2 = 0;

    for i in tokens {
        if i == 1 {
            count_1 += 1;
        } else {
            count_2 += 1;
        }
    }

    if count_1 > count_2 {
        print!("1");
    } else {
        print!("2");
    }
}
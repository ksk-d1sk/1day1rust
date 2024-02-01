use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let tokens: Vec<usize> = input.split_ascii_whitespace().flat_map(str::parse).collect();

    let temp = tokens.windows(2);

    if temp.clone().all(|sli| sli[0] < sli[1]) {
        print!("Fish Rising");
    } else if temp.clone().all(|sli| sli[0] > sli[1]) {
        print!("Fish Diving");
    } else if temp.clone().all(|sli| sli[0] == sli[1]) {
        print!("Fish At Constant Depth");
    } else {
        print!("No Fish");
    }

    // macro_rules! next {
    //     () => { tokens.next().unwrap() };
    //     ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    // }
}
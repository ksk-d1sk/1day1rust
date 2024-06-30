use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let tokens = input.bytes();

    let arr = [b'U', b'C', b'P', b'C'];
    let mut i = 0;

    for s in tokens {
        if i == 4 {
            break;
        } else if s == arr[i] {
            i += 1;
        }
    }

    if i == 4 {
        print!("I love UCPC");
    } else {
        print!("I hate UCPC");
    }
}
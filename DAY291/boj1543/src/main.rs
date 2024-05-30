use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.lines();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let a = next!().as_bytes();
    let b = next!().as_bytes();
    let al = a.len();
    let bl = b.len();
    let mut i = 0;
    let mut count = 0;

    while i < al {
        let mut check = true;

        for j in 0..bl {
            if a.get(i + j) != b.get(j) {
                check = false;
                break;
            }
        }

        if check {
            count += 1;
            i += bl;
        } else {
            i += 1;
        }
    }

    print!("{count}");
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = 0;
    let mut v = vec![b' '];

    v.extend(next!().bytes());

    let l = v.len();

    for i in 1..v.len() {
        if v[i] == b'Y' {
            answer += 1;

            for j in (1..).map(|j| i * j).take_while(|j| *j < l) {
                if v[j] == b'Y' {
                    v[j] = b'N';
                } else {
                    v[j] = b'Y';
                }
            }
        }
    }

    print!("{answer}");
}
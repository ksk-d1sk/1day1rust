use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let v = Vec::from_iter((0..n).map(|_| next!(usize)));
    let mut fruits_count = [0; 10];
    let mut front = 0;
    let mut back = 0;
    let mut answer = 0;

    loop {
        let mut count = 0;

        for i in 1..=9 {
            if fruits_count[i] > 0 {
                count += 1;
            }
        }

        if back == n {
            if count <= 2 {
                answer = answer.max(back - front);
            }
            break;
        } else if count > 2 {
            fruits_count[v[front]] -= 1;
            front += 1;
        } else {
            answer = answer.max(back - front);
            fruits_count[v[back]] += 1;
            back += 1;
        }
    }

    print!("{answer}");
}
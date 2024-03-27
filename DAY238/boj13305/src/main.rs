use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut drive = vec![0];
    drive.extend((1..n).map(|_| next!(u64)));
    let oil: Vec<_> = (0..n).map(|_| next!(u64)).collect();
    let mut answer = 0;
    let mut cur = 0;
    let mut next_city = 1;

    while next_city < n {
        if oil[next_city] < oil[cur] || next_city == n - 1 {
            answer += ((cur + 1)..=next_city).map(|i| drive[i]).sum::<u64>() * oil[cur];
            cur = next_city;
        }

        next_city += 1;
    }

    print!("{answer}");
}
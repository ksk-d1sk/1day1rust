use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let arr = Vec::from_iter((0..n).map(|_| next!(u32)));
    let mut lis = vec![0];

    for i in 0..n {
        if let Err(idx) = lis.binary_search(&arr[i]) {
            if lis.get(idx).is_some() {
                lis[idx] = lis[idx].min(arr[i]);
            } else {
                lis.push(arr[i]);
            }
        }
    }

    print!("{}", 1 + n - lis.len());
}
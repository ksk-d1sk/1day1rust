use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut arr = [0; 26];

    (0..n)
        .map(|_| get!().as_bytes()[0] as usize - 97)
        .for_each(|i| arr[i] += 1);

    let mut answer: String = arr
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| {
            (x >= 5).then_some((b'a' + i as u8) as char)
        })
        .collect();

    if answer.is_empty() {
        answer = "PREDAJA".to_string();
    }

    print!("{answer}");
}
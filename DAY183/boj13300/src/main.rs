use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = 0;
    let mut v = [[0; 6]; 2];
    let (n, k) = get!(u16, u16);

    for (s, y) in (0..n).map(|_| get!(usize, usize)) {
        v[s][y - 1] += 1;
    }
    
    for sv in v {
        for nof in sv {
            answer += (nof + k - 1) / k;
        }
    }

    print!("{answer}");
}
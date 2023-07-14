use std::io::Read;

fn main() {
    // let mut f = std::fs::File::open("input.txt").unwrap();
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // f.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ( $(input.next().unwrap().parse::<$t>().unwrap()),+ ) };
    }

    let (n, m) = get!(usize, usize);
    let mut v = Vec::with_capacity(n);

    for _ in 0..n {
        v.push(get!().as_bytes());
    }

    print!("{}", solve(n, m, v));
}

fn solve(n: usize, m: usize, v: Vec<&[u8]>) -> usize {
    let mut l = n.min(m);

    'a: loop {
        for i in 0..=(n - l) {
            for j in 0..=(m - l) {
                if (v[i + l - 1][j], v[i][j + l - 1], v[i + l - 1][j + l - 1]) == (v[i][j], v[i][j], v[i][j]) {
                    break 'a l * l;
                }
            }
        }
        l -= 1;
    }
}
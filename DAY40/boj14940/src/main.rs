use std::io::{self, Read};

fn main() {
    let mut f = std::fs::File::open("input.txt").unwrap();
    let mut input = String::new();
    // let input = io::read_to_string(io::stdin()).unwrap();
    f.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let mut map: Vec<Vec<i32>> = Vec::with_capacity(n);
    let mut n_xy = (0, 0);

    for x in 0..n {
        map.push((0..m).map(|y| {
            let temp = get!(i32);
            if temp == 2 {
                n_xy = (x, y);
            }
            temp
        }).collect());
    }

    println!("{:?}", n_xy);
}

use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut arr1 = [false; 1001];
    let mut arr2 = [false; 1001];

    for (x, y) in (0..3).map(|_| get!(usize, usize)) {
        arr1[x] ^= true;
        arr2[y] ^= true;
    }

    let (ans1, ans2) = (
        arr1.into_iter()
            .enumerate()
            .filter_map(|(i, b)| b.then_some(i))
            .next()
            .unwrap(),

        arr2.into_iter()
            .enumerate()
            .filter_map(|(i, b)| b.then_some(i))
            .next()
            .unwrap()
    );

    print!("{ans1} {ans2}");
}

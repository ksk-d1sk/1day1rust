use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace().skip(1);

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut a = get!(u64);
    let mut arr: Vec<u64> = input.flat_map(str::parse).collect();

    arr.sort_unstable();

    for i in arr {
        if a > i {
            a += i;
        } else {
            print!("No");
            return;
        }
    }

    print!("Yes");
}
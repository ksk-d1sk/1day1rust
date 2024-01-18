use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(u16);
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    let mut axis = 0;

    for (x, y) in (0..n).map(|_| get!(i32, i32)) {
        if x > 0 && y > 0 {
            q1 += 1;
        } else
        if x < 0 && y > 0 {
            q2 += 1;
        } else
        if x < 0 && y < 0 {
            q3 += 1;
        } else
        if x > 0 && y < 0 {
            q4 += 1;
        } else
        {
            axis += 1;
        }
    }

    print!("Q1: {q1}\nQ2: {q2}\nQ3: {q3}\nQ4: {q4}\nAXIS: {axis}");
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    next!();
    let s = next!();
    let (d, m) = next!(usize, usize);
    let mut hyu = [0_usize; 3];
    let mut energy = 0;
    let mut combo = 0;

    for b in s.bytes() {
        match b {
            b'H' => {
                hyu[0] += 1;
            }
            b'Y' => {
                hyu[1] += 1;
            }
            b'U' => {
                hyu[2] += 1;
            }
            _ => {
                combo += 1;
            }
        }

        if matches!(b, b'H' | b'Y' | b'U') {
            energy += (d * combo).min(d + m);
            combo = 0;
        }
    }

    energy += (d * combo).min(d + m);

    if energy != 0 {
        println!("{energy}");
    } else {
        println!("Nalmeok");
    }

    let hyu_count = hyu.iter().copied().min().unwrap();

    if hyu_count != 0 {
        print!("{hyu_count}");
    } else {
        print!("I love HanYang University");
    }
}
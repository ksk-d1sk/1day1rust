use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
    }

    let s = get!().as_bytes();
    let t = get!().as_bytes();
    let mut i = 0;
    let mut j = t.len() - 1;
    let mut toggle = true;

    while s.len() < 1 + j - i {
        if toggle {
            if t[j] == b'B' {
                toggle = !toggle;
            } else if t[j] != b'A' {
                break;
            }
            j -= 1;
        } else {
            if t[i] == b'B' {
                toggle = !toggle;
            } else if t[i] != b'A' {
                break;
            }
            i += 1;
        }
    }

    print!(
        "{}",
        compare(
            s, t,
            if toggle {
                Box::new((i..=j).enumerate())
            } else {
                Box::new((i..=j).rev().enumerate())
            }
        )
    );
}

fn compare(s: &[u8], t: &[u8], iter: Box<dyn Iterator<Item=(usize, usize)>>) -> u8 {
    let mut ret = 1;

    for (i, j) in iter {
        if !s.get(i).is_some_and(|x| *x == t[j]) {
            ret = 0;
            break;
        }
    }

    ret
}
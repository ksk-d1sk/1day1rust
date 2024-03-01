use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(u16);
    for _ in 0..t {
        let v = next!().as_bytes();
        let mut a = 0;
        let mut chk = false;
    
        for i in 0..v.len() {
            if v[i] == b'!' {
                a += 1;
            } else {
                chk = v[i] == b'1';
                break;
            }
        }

        if v.len() - a - 1 > 0 {
            chk = true;
        }

        for _ in 0..a {
            chk = !chk;
        }

        if chk {
            output.push_str("1\n");
        } else {
            output.push_str("0\n");
        }
    }

    print!("{output}");
}
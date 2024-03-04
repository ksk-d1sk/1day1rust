use std::io::*;

fn main() {
    let mut stdin = stdin().lock();

    macro_rules! next {
        () => {{
            let mut buf = String::new();
            stdin.read_line(&mut buf).unwrap();
            buf
        }};
        ( $t:ty ) => {{
            let mut buf = String::new();
            stdin.read_line(&mut buf).unwrap();
            buf.trim().parse::<$t>().unwrap()
        }}
    }

    let n = next!(usize);
    let names: Vec<_> = (0..n).map(|_| next!()).collect();

    for i in 0..n {
        for _ in 0..2 {
            println!("? {}", names[i]);
            if next!(u8) == 1 {
                print!("! {}", names[i]);
                return;
            }
        }
    }
}
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        () => { input.next().unwrap() };
    }

    print!(
        "{}",
        loop {
            let n = get!();

            if n == "0" {
                break output;
            }

            let mut temp = 0_i32;
            for &b in n.as_bytes() {
                temp = (temp * 10 - 48 + b as i32) % 42;
            }

            output.push_str(if temp == 0 { "PREMIADO\n" } else { "TENTE NOVAMENTE\n" });
        }
    );
}
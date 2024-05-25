use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    let a = br"
 1234567890-=WERTYUIOP[]\SDFGHJKL;'XCVBNM,./";
    let b = br"
 `1234567890-QWERTYUIOP[]ASDFGHJKL;ZXCVBNM,.";

    for c in input.bytes() {
        for i in 0..45 {
            if a[i] == c {
                output.push(b[i] as char);
                break;
            }
        }
    }

    print!("{output}");
}
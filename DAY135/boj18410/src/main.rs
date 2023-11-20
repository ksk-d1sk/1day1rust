use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut v: Vec<_> = buf.split_ascii_whitespace().skip(2).collect();

    v.sort_unstable_by(|a, b|
        if a.len() != b.len() {
            a.len().cmp(&b.len())
        } else {
            a.cmp(b)
        }
    );

    print!(
        "{}",
        v.join("\n")
    );
}

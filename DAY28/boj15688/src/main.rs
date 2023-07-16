use std::io::Read;
use std::fmt::Write;

fn main() {
    // let mut f = std::fs::File::open("input.txt").unwrap();
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // f.read_to_string(&mut input).unwrap();

    let mut v = input.split_ascii_whitespace().skip(1).flat_map(str::parse).collect::<Vec<i32>>();
    v.sort_unstable();

    for i in v.iter() {
        let _ = write!(output, "{} ", i);
    }

    print!("{output}");
}

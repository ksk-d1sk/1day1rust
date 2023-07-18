use std::io::Read;
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut v: Vec<f64> = input.split_ascii_whitespace().skip(1).flat_map(str::parse).collect();
    v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    for elem in v.iter().take(7) {
        let _ = writeln!(output, "{:.03}", elem);
    }

    print!("{output}");
}
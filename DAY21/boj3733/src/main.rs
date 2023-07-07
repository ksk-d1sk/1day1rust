use std::io::Read;
use std::fmt::Write;

fn main() {
    // let mut f = std::fs::File::open("input.txt").unwrap();
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // f.read_to_string(&mut input).unwrap();

    for mut input in input.lines().map(|e| e.split_ascii_whitespace().flat_map(str::parse::<u32>)) {
        let t = input.next().unwrap() + 1;
        let _ = writeln!(output, "{}", input.next().unwrap() / t);
    }

    print!("{output}");
}

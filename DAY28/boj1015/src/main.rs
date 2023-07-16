use std::io::Read;
use std::fmt::Write;

fn main() {
    // let mut f = std::fs::File::open("input.txt").unwrap();
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // f.read_to_string(&mut input).unwrap();

    let mut v = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(str::parse)
        .enumerate()
        .collect::<Vec<(usize, u16)>>();

    v.sort_by_key(|e| e.1);

    let mut temp = vec![0; v.len()];

    for (i, (j, _)) in v.iter().enumerate() {
        temp[*j] = i;
    }

    for i in 0..v.len() {
        let _ = write!(output, "{} ", temp[i]);
    }

    print!("{output}");
}

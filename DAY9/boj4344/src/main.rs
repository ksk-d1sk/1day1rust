use std::io::Read;
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    for input in input.lines().skip(1) {
        let line: Vec<f64> = input.split_ascii_whitespace().flat_map(str::parse).collect();
        let (n, score_list) = (line[0], &line[1..]);
        let average = score_list.iter().sum::<f64>() / n;
        writeln!(
            output,
            "{:.3}%",
            score_list.iter().filter(|&&score| score > average).count() as f64 / n * 100.0 + 0.0001
        ).unwrap();
    }

    print!("{output}");
}
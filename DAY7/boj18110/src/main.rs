use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut v: Vec<f64> = input.split_ascii_whitespace().skip(1).flat_map(str::parse).collect();
    v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    let l = (v.len() as f64 * 0.15 + 0.5) as usize;

    println!("{}", avg(&v[l..(v.len() - l)]));
}

fn avg(arr: &[f64]) -> u32 {
    let len = arr.len() as f64;
    (arr.iter().sum::<f64>() / len + 0.5) as u32
}
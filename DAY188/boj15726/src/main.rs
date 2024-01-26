use std::io::*;

fn main() {
    let buf = read_to_string(stdin()).unwrap();
    let v: Vec<f64> = buf.split_ascii_whitespace().flat_map(str::parse).collect();

    let a = v[0] * v[1] / v[2];
    let b = v[0] / v[1] * v[2];

    print!("{}", a.max(b) as i32);
}
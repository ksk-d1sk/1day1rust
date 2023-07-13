use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    print!("{}", input.split_ascii_whitespace().skip(1).flat_map(str::parse::<i32>).sum::<i32>());
}

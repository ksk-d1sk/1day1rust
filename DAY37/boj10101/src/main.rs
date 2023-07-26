use std::io::Read;
use std::cmp::Ordering::*;
use std::collections::HashSet;

fn main() {
    // let mut f = std::fs::File::open("input.txt").unwrap();
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // f.read_to_string(&mut input).unwrap();
    let input: Vec<u16> = input.split_ascii_whitespace().flat_map(str::parse).collect();

    if input.iter().sum::<u16>() == 180 {
        print!(
            "{}",
            match input.into_iter().collect::<HashSet<u16>>().len().cmp(&2) {
                Greater => "Scalene",
                Equal => "Isosceles",
                Less => "Equilateral",
            }
        );
    } else {
        print!("Error");
    }
}

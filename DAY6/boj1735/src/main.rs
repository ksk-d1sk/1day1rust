use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<u32>);

    let mut get = || input.next().unwrap();

    let a_numerator = get();
    let a_denominator = get();
    let b_numerator = get();
    let b_denominator = get();

    let numerator = a_numerator * b_denominator + b_numerator * a_denominator;
    let denominator = a_denominator * b_denominator;

    let gcd = euclidean(numerator, denominator);

    println!("{} {}", numerator / gcd, denominator / gcd);
}

fn euclidean(a: u32, b: u32) -> u32 {
    let (max, min) =
        if a > b { (a ,b) }
        else     { (b, a) };

    let r = max % min;

    if r == 0 {
        min
    } else {
        euclidean(min, r)
    }
}
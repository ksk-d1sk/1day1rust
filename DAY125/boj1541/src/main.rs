use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut formula_iter = buf.trim().split('-');
    let mut answer = calc(formula_iter.next().unwrap());

    for formula in formula_iter {
        answer -= calc(formula);
    }

    print!("{answer}");
}

fn calc(formula: &str) -> i32 {
    formula.split('+').flat_map(str::parse::<i32>).sum()
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let sum = (0..n)
        .map(|_| match next!() {
            "Poblano" => 1500,
            "Mirasol" => 6000,
            "Serrano" => 15500,
            "Cayenne" => 40000,
            "Thai" => 75000,
            "Habanero" => 125000,
            _ => unreachable!(),
        })
        .sum::<usize>();

    print!("{sum}");
}
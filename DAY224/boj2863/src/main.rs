use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b, c, d) = next!(f64, f64, f64, f64);
    let mut max = f64::MIN;
    let mut answer = 0;

    for (i, (a, b, c, d)) in [
        (a, b, c, d),
        (c, a, d, b),
        (d, c, b, a),
        (b, d, a, c),
    ].into_iter().enumerate() {
        let temp = a / c + b / d;
        if temp > max {
            max = temp;
            answer = i;
        }
    }

    print!("{answer}");
}
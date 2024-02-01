use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    for x in (0..).map(|_| next!(f64)).take_while(|&x| x >= 0.0) {
        let temp1 = (x * 100.0) as u32;
        let temp2 = (x * 16.7) as u32;
        println!("Objects weighing {}.{:02} on Earth will weigh {}.{:02} on the moon.", temp1 / 100, temp1 % 100, temp2 / 100, temp2 % 100);
    }
}
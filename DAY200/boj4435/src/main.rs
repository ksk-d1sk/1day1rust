use std::cmp::Ordering::*;
use std::fmt::Write;
use std::io::*;

const GANDALF: [u32; 6] = [1, 2, 3, 3, 4, 10];
const SAURON: [u32; 7] = [1, 2, 2, 2, 3, 5, 10];

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(usize);
    for i in 1..=t {
        let gandalf = GANDALF.into_iter().zip((0..6).map(|_| next!(u32))).fold(0, combat_power);
        let sauron = SAURON.into_iter().zip((0..7).map(|_| next!(u32))).fold(0, combat_power);

        let _ = writeln!(
            output,
            "Battle {i}: {}",
            match gandalf.cmp(&sauron) {
                Less => "Evil eradicates all trace of Good",
                Equal => "No victor on this battle field",
                Greater => "Good triumphs over Evil",
            }
        );
    }

    print!("{output}");
}

fn combat_power(acc: u32, x: (u32, u32)) -> u32 {
    acc + (x.0 * x.1)
}
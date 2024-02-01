use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let y = next!(usize);
    let archive = ["ITMO", "SPbSU", "SPbSU", "ITMO", "ITMO",
                   "SPbSU", "ITMO", "ITMO", "ITMO", "ITMO",
                   "ITMO", "PetrSU, ITMO", "SPbSU", "SPbSU",
                   "ITMO", "ITMO", "ITMO", "ITMO", "SPbSU",
                   "ITMO", "ITMO", "ITMO", "ITMO", "SPbSU", "ITMO"];

    print!("{}", archive[y - 1995]);
}
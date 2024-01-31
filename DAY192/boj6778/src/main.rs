use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let antenna = next!(u8);
    let eyes = next!(u8);

    if antenna >= 3 && eyes <= 4{
        println!("TroyMartian");
    }

    if antenna <= 6 && eyes >= 2 {
        println!("VladSaturnian");
    }

    if antenna <= 2 && eyes <= 3 {
        println!("GraemeMercurian");
    }
}
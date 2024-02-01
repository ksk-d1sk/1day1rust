use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    print!(
        "{}",
        match next!() {
            "fdsajkl;" | "jkl;fdsa" => "in-out",
            "asdf;lkj" | ";lkjasdf" => "out-in",
            "asdfjkl;" => "stairs",
            ";lkjfdsa" => "reverse",
            _ => "molu",
        }
    );
}
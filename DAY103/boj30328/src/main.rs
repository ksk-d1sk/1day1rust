use std::io;

fn main() {
    print!(
        "{}",
        io::read_to_string(io::stdin())
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap() * 4000
    );
}

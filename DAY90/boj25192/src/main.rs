use std::collections::HashSet;
use std::io;

fn main() {
    print!(
        "{}",
        io::read_to_string(io::stdin())
            .unwrap()
            .split("ENTER")
            .skip(1)
            .map(|name| name.split_ascii_whitespace().collect::<HashSet<_>>().len())
            .sum::<usize>()
    );
}

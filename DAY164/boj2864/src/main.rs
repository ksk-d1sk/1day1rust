use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
    }

    let a = get!();
    let b = get!();

    let min_a = a.chars()
        .map(|c| if c == '6' { '5' } else { c })
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    let min_b = b.chars()
    .map(|c| if c == '6' { '5' } else { c })
    .collect::<String>()
    .parse::<u32>()
    .unwrap();

    let max_a = a.chars()
        .map(|c| if c == '5' { '6' } else { c })
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    let max_b = b.chars()
        .map(|c| if c == '5' { '6' } else { c })
        .collect::<String>()
        .parse::<u32>()
        .unwrap();

    print!("{} {}", min_a + min_b, max_a + max_b);
}

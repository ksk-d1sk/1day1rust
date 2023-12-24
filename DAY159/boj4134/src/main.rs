use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();
    let mut output = String::new();

    for n in input.skip(1).flat_map(str::parse::<u32>) {
        let _ = writeln!(output, "{}", solve(n));
    }

    print!("{output}");
}

fn solve(n: u32) -> u32 {
    let mut i = n.max(2);
    loop {
        let mut check = true;

        for j in 2.. {
            if j * j > i { break }
            if i % j == 0 {
                check = false;
                break;
            }
        }

        if check {
            break i;
        }
        
        i += 1;
    }
}
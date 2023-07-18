use std::io::Read;
use std::collections::BTreeSet;
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    macro_rules! split {
        ( $e:expr ) => {{
            let mut iter = $e.split_ascii_whitespace();
            (iter.next().unwrap(), iter.next().unwrap())
        }};
    }

    let mut set = BTreeSet::new();

    for (name, cmd) in input.lines().skip(1).map(|e| split!(e)) {
        match cmd {
            "enter" => set.insert(name),
            "leave" => set.remove(name),
            _ => unreachable!(),
        };
    }

    for elem in set.iter().rev() {
        let _ = writeln!(output, "{}", elem);
    }

    print!("{output}");
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.lines();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
    }

    let s = next!().chars();
    let mut temp = '!';

    for c in s {
        if c != temp {
            temp = c;
            output.push(c);
        }
    }

    print!("{output}");
}
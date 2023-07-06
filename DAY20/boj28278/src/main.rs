use std::io::Read;
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut stack = Vec::with_capacity(1_000_000);

    for line in input.lines().skip(1) {
        if let Some(data) = line.get(2..) {
            stack.push(data);
        } else {
            let _ = match line {
                "2" => writeln!(output, "{}", stack.pop().unwrap_or("-1")),       // pop
                "3" => writeln!(output, "{}", stack.len()),                       // size
                "4" => writeln!(output, "{}", u8::from(stack.is_empty())),        // empty
                "5" => writeln!(output, "{}", stack.last().unwrap_or(&"-1")),     // top
                _ => panic!(),
            };
        }
    }

    print!("{output}");
}
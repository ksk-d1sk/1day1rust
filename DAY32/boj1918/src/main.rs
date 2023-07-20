fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let input = input.trim().chars();

    let mut stack = Vec::new();

    for i in input {
        match i {
            '+' | '-' => {
                while let Some(&c) = stack.last() {
                    if c == '(' {
                        break;
                    } else {
                        output.push(c);
                        stack.pop();
                    }
                }
                stack.push(i);
            },
            '*' | '/' => {
                while let Some(&c) = stack.last() {
                    match c {
                        '(' | '+' | '-' => break,
                        _ => {
                            output.push(c);
                            stack.pop();
                        }
                    }
                }
                stack.push(i);
            },
            '(' => stack.push(i),
            ')' => {
                while let Some(c) = stack.pop() {
                    if c == '(' {
                        break;
                    } else {
                        output.push(c);
                    }
                }
            }
            _ => output.push(i),
        }
    }

    while let Some(c) = stack.pop() {
        output.push(c);
    }

    print!("{output}");
}
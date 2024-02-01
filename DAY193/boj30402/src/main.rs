use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let tokens = input.split_ascii_whitespace();

    let mut answer = "";

    for c in tokens {
        match c {
            "w" => {
                answer = "chunbae";
                break;
            },
            "b" => {
                answer = "nabi";
                break;
            },
            "g" => {
                answer = "yeongcheol";
                break;
            }
            _ => {},
        }
    }

    print!("{answer}");
}
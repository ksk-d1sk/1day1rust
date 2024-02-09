use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut account = 0;

    while let Some(item) = tokens.next() {
        account += match item {
            "Paper" => 5799,
            "Printer" => 12050,
            "Planners" => 3125,
            "Binders" => 2250,
            "Calendar" => 1095,
            "Notebooks" => 1120,
            "Ink" => 6695,
            _ => 0,
        }
    }

    print!("${}.{:02}", account / 100, account % 100);
}
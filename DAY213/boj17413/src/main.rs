use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.lines();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let s = next!();
    let mut answer = String::new();
    let mut temp_str = String::new();
    let mut tag_opened = false;

    for c in s.chars() {
        match c {
            '<' => {
                tag_opened = true;
                flush(&mut answer, &mut temp_str);
                answer.push(c);
            },
            '>' => {
                tag_opened = false;
                answer.push(c);
            },
            ' ' if !tag_opened => {
                flush(&mut answer, &mut temp_str);
                answer.push(c);
            },
            _ if tag_opened => answer.push(c),
            _ => temp_str.push(c),
        }
    }

    flush(&mut answer, &mut temp_str);

    print!("{answer}");
}

fn flush(l: &mut String, r: &mut String) {
    while let Some(c) = r.pop() {
        l.push(c);
    }
}
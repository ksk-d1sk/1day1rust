use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut last = ("0", 0);
    let mut desc = Vec::new();
    let mut check = true;

    for ticket in (0..n*5).map(|_| {
        let mut temp = next!().split('-');
        (temp.next().unwrap(), temp.next().unwrap().parse::<u16>().unwrap())
    }) {
        while !desc.is_empty() && ticket > *desc.last().unwrap() {
            let temp = desc.pop().unwrap();
            if ticket > last {
                last = temp;
            } else {
                check = false;
            }
        }

        desc.push(ticket);
    }

    if last > *desc.last().unwrap() {
        check = false;
    }

    if check {
        print!("GOOD");
    } else {
        print!("BAD");
    }
}
use std::io::Read;

fn main() {
    // let mut f = std::fs::File::open("input.txt").unwrap();
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // f.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $t:ty ) => { input.next().unwrap().parse::<$t>().unwrap() };
    }

    let n = get!(usize);
    let mut v = Vec::with_capacity(n);
    let notation = get!().as_bytes();
    let mut stack = Vec::new();

    for i in input {
        v.push(i.parse::<f64>().unwrap());
    }

    for e in notation {
        let temp = match e {
            b'+' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                b + a
            },
            b'-' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                b - a
            },
            b'*' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                b * a
            },
            b'/' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                b / a
            },
            _ => {
                stack.push(v[(*e - b'A') as usize]);
                continue;
            },
        };

        stack.push(temp);
    }

    print!("{:.02}", stack.pop().unwrap());
}
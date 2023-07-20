use std::io::Read;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    for ice_cream in input.split_ascii_whitespace().skip(1) {
        output.push((u8::from(solve(ice_cream)) + b'0') as char);
        output.push('\n');
    }

    print!("{output}");
}

fn solve(ice_cream: &str) -> bool {
    let mut answer = false;
    let x = if ice_cream.len() % 3 == 0 {
        ice_cream.len() / 3
    } else {
        ice_cream.len() / 3 + 1
    };

    let s = &ice_cream[..x];

    if ice_cream[x..].starts_with(tail(&rev(s))) {
        let l = x * 2 - 1;
        answer = &ice_cream[l..] == s || &ice_cream[l..] == tail(s);
    }

    if !answer && ice_cream[x..].starts_with(&rev(s)) {
        let l = x * 2;
        answer = &ice_cream[l..] == s || &ice_cream[l..] == tail(s);
    }

    answer
}

fn rev(s: &str) -> String {
    s.chars().rev().collect()
}

fn tail(s: &str) -> &str {
    &s[1..]
}
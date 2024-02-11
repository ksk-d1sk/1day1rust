use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (x, y) = next!(usize, usize);
    let l = lcm(x, y);
    let mut v = vec!['0'; l + 1];

    for i in (1..).map(|i| l / x * i).take_while(|&i| i <= l) {
        v[i] = '1';
    }

    for i in (1..).map(|i| l / y * i).take_while(|&i| i <= l) {
        if v[i] == '1' {
            v[i] = '3';
        } else {
            v[i] = '2';
        }
    }

    for c in v.into_iter().filter(|&c| c != '0') {
        output.push(c);
    }

    print!("{output}");
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a.max(b), a.min(b))
}

fn gcd(a: usize, b: usize) -> usize {
    let m = a % b;
    if m == 0 {
        b
    } else {
        gcd(b, m)
    }
}
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let a = next!();
    let b = next!();
    let v = Vec::from_iter((0..n).map(|_| (next!(), next!(u32))));
    let i = find(a, &v);
    let j = find(b, &v);

    let mut clone_v = v.clone();
    clone_v[i].1 += 3;
    sort(&mut clone_v);
    print!("{} ", find(a, &clone_v) + 1);

    let mut clone_v = v.clone();
    clone_v[i].1 += 1;
    clone_v[j].1 += 1;
    sort(&mut clone_v);
    print!("{} ", find(a, &clone_v) + 1);

    let mut clone_v = v.clone();
    clone_v[j].1 += 3;
    sort(&mut clone_v);
    print!("{}", find(a, &clone_v) + 1);
}

fn find(name: &str, v: &[(&str, u32)]) -> usize {
    for i in 0..v.len() {
        if v[i].0 == name {
            return i;
        }
    }
    unreachable!();
}

fn sort(v: &mut Vec<(&str, u32)>) {
    v.sort_unstable_by(|a, b| {
        if a.1 != b.1 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(b.0)
        }
    });
}
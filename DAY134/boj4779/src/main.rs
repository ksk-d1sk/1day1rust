use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse);
    let mut output = String::new();

    for i in input {
        let mut ans = vec!['-'; 3_usize.pow(i)];

        donut(&mut ans);

        let _ = writeln!(output, "{}", String::from_iter(ans));
    }

    print!("{output}");
}

fn donut(slice: &mut [char]) {
    let div_3 = slice.len() / 3;

    if div_3 != 0 {
        let mut iter = slice.chunks_mut(div_3);
        let first = iter.next().unwrap();

        if let Some(second) = iter.next() {
            let third = iter.next().unwrap();
            for c in second {
                *c = ' ';
            }

            donut(first);
            donut(third);
        }
    }
}
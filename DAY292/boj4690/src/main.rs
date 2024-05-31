use std::fmt::Write;

fn main() {
    let mut output = String::new();
    let v = Vec::from_iter((0..=100).map(|i| i * i * i));

    for a in 3..=100 {
        for b in 2..a {
            for c in b..a {
                for d in c..a {
                    if v[a] == v[b] + v[c] + v[d] {
                        let _ = writeln!(output, "Cube = {a}, Triple = ({b},{c},{d})");
                    }
                }
            }
        }
    }

    print!("{output}");
}
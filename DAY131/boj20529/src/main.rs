use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        () => { input.next().unwrap() };
        ($($t:ty),+) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    for _ in 0..get!(u8) {
        let n = get!(usize);
        let v = (0..n).map(|_| get!()).collect();

        let _ = writeln!(output, "{}", solve(n, v));
    }

    print!("{output}");
}

fn solve(n: usize, mbti_v: Vec<&str>) -> usize {
    if n > 32 { return 0 }

    let mut answer = usize::MAX;

    for i in 0..(n - 2) {
        for j in (i + 1)..(n - 1) {
            for k in (j + 1)..n {
                let mut sum = 0;
                sum += psychological_distance(mbti_v[i], mbti_v[j]);
                sum += psychological_distance(mbti_v[i], mbti_v[k]);
                sum += psychological_distance(mbti_v[j], mbti_v[k]);

                answer = answer.min(sum);
            }
        }
    }

    answer
}

fn psychological_distance(lhs: &str, rhs: &str) -> usize {
    lhs.chars().zip(rhs.chars()).filter(|(l, r)| l != r).count()
}
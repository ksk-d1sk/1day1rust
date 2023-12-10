use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();

    let mut v: Vec<u16> = input.flat_map(str::parse).collect();
    let sum_sub_100 = v.iter().sum::<u16>() - 100;
    let (mut a, mut b) = (0, 0);

    v.sort_unstable();

    'a: for i in 0..8 {
        for j in i..9 {
            if v[i] + v[j] == sum_sub_100 {
                (a, b) = (v[i], v[j]);
                break 'a;
            }
        }
    }

    for elem in v {
        if elem != a && elem != b {
            println!("{elem}");
        }
    }
}
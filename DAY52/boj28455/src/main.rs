use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();

    let mut v: Vec<u16> = input.skip(1).flat_map(str::parse).collect();
    v.sort_unstable_by(|a, b| b.cmp(a));
    let characters = v.into_iter().take(42);

    let level_sum: u16 = characters.clone().sum();
    let mut stats_sum = 0;

    for level in characters {
        if 250 <= level {
            stats_sum += 5;
        } else if 200 <= level {
            stats_sum += 4;
        } else if 140 <= level {
            stats_sum += 3;
        } else if 100 <= level {
            stats_sum += 2;
        } else if 60 <= level {
            stats_sum += 1;
        }
    }

    print!("{} {}", level_sum, stats_sum);
}

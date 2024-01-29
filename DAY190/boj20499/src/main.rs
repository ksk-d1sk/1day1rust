use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let kda: Vec<u8> = input.trim().split('/').flat_map(str::parse).collect();

    print!(
        "{}",
        if kda[0] + kda[2] < kda[1] || kda[1] == 0 {
            "hasu"
        } else {
            "gosu"
        }
    )
}
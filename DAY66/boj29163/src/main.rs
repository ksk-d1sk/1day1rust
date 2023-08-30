use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().skip(1).flat_map(str::parse::<usize>);

    let mut arr = [0_u32; 2];

    for i in input {
        arr[i & 1] += 1;
    }

    print!(
        "{}",
        if arr[0] > arr[1] {
            "Happy"
        } else {
            "Sad"
        }
    )
}

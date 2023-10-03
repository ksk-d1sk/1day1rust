fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: u32 = buf.trim().parse().unwrap();

    let is_contain_seaven = buf.contains('7');
    let is_divisible_seaven = n % 7 == 0;

    print!(
        "{}",
        if is_contain_seaven && is_divisible_seaven {
            3
        } else if is_contain_seaven {
            2
        } else if is_divisible_seaven {
            1
        } else {
            0
        }
    );
}

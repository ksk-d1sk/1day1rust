fn main() {
    assert_eq!(triangle(0), 0);
    assert_eq!(triangle(1), 1);
    assert_eq!(triangle(2), 3);
    assert_eq!(triangle(3), 6);
    assert_eq!(triangle(4), 10);
    assert_eq!(triangle(5), 15);
    assert_eq!(triangle(10), 55);
    assert_eq!(triangle(20), 210);
}

fn triangle(n: u64) -> u64 {
    (1..=n).sum()
}
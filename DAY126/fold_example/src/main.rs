fn main() {
    let a = [5, 6, 7, 8, 9, 10];

    assert_eq!(a.iter().fold(0, |n, _| n + 1), 6);      // count
    assert_eq!(a.iter().fold(0, |n, i| n + i), 45);     // sum
    assert_eq!(a.iter().fold(1, |n, i| n * i), 151200); // product

    assert_eq!(a.iter().copied().fold(i32::MIN, std::cmp::max), 10); // max

    let a = ["Pack", "my", "box", "with", "five", "dozen", "liquor", "jugs"];
    let numbers = [1, 2, 3, 4, 5];

    let pangram = a.iter().fold(String::new(), |s, w| s + w + " ");
    assert_eq!(pangram, "Pack my box with five dozen liquor jugs ");

    let result = numbers.iter().fold(String::from("0"), |acc, x| format!("({} + {})", acc, x));
    assert_eq!(result, "(((((0 + 1) + 2) + 3) + 4) + 5)");

    let weird_pangram = a.iter().rfold(String::new(), |s, w| s + w + " ");
    assert_eq!(weird_pangram, "jugs liquor dozen five with box my Pack ");

    let result = numbers.iter().rfold(String::from("0"), |acc, x| format!("({} + {})", x, acc));
    assert_eq!(result, "(1 + (2 + (3 + (4 + (5 + 0)))))");
}

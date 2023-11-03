use std::iter::repeat;

fn main() {
    let v1: Vec<(usize, char)> = (0_usize..).zip("ABCD".chars()).collect();
    let v2: Vec<(usize, char)> = "ABCD".chars().enumerate().collect();

    assert_eq!(v1, [(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);
    assert_eq!(v1, v2);

    let endings = ["once", "twice", "chicken soup with rice"];
    let rhyme: Vec<_> = repeat("going")
        .zip(endings)
        .collect();
    assert_eq!(rhyme, [("going", "once"),
                       ("going", "twice"),
                       ("going", "chicken soup with rice")]);
}

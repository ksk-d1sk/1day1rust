use std::collections::HashSet;

fn main() {
    let squaares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>)
        = squaares.iter().partition(|&n| n & (n - 1) == 0);
    assert_eq!(powers_of_two.len(), 3);
    assert_eq!(impure.len(), 4);

    let (upper, lower): (String, String)
        = "Great Teacher Onizuka".chars().partition(|&c| c.is_uppercase());
    assert_eq!(upper, "GTO");
    assert_eq!(lower, "reat eacher nizuka");
}

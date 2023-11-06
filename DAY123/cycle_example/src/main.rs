use std::iter::{once, repeat};

fn main() {
    let dirs = ["North", "East", "South", "West"];
    let mut spin = dirs.iter().copied().cycle();
    assert_eq!(spin.next(), Some("North"));
    assert_eq!(spin.next(), Some("East"));
    assert_eq!(spin.next(), Some("South"));
    assert_eq!(spin.next(), Some("West"));
    assert_eq!(spin.next(), Some("North"));
    assert_eq!(spin.next(), Some("East"));
    assert_eq!(spin.next(), Some("South"));
    assert_eq!(spin.next(), Some("West"));

    let fizzes = repeat("").take(2).chain(once("Fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("Buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);
    let fizz_buzz = (1..=100).zip(fizzes_buzzes)
        .map(|tuple| {
            match tuple {
                (i, ("", "")) => i.to_string(),
                (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
            }
        });

    for line in fizz_buzz {
        println!("{}", line);
    }
}

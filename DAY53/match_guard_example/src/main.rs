fn main() {
    let a = Some(3);

    match a {
        None => println!("a is None"),
        Some(h) if h == 3 => println!("a is 3"),
        Some(h) => println!("a == {}", h),
    }
}

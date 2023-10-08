fn main() {
    let a: i32 = -123;
    let b: u32 = a.try_into().unwrap_or(0);
    println!("{}", b);


    let a: i64 = i32::MAX as i64 + 99_999;
    let b: i32 = a.try_into().unwrap_or(i32::MAX);
    println!("{}", b);
}

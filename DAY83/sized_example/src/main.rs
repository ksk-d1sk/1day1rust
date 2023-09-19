use std::fmt::Display;

/*
// 러스트의 제네릭 타입은 컴파일 시점에 크기가 정해진다
fn print_str<T: Display>(value: &T) {
    println!("print: {}", value);
}

fn main() {
    let string = "Hello, World!";

    print_str(string);  // str의 타입을 컴파일 타임에 알지 못하므로 오류가 발생한다
}
*/

// 제네릭 타입이 동적으로 크기를 추적하도록 ?Sized 트레이트를 추가한다
fn print_str<T: Display + ?Sized>(value: &T) {
    println!("print: {}", value);
}

fn main() {
    let string = "Hello, World!";

    print_str(string);  // str의 크기를 동적으로 검사하므로 오류가 발생하지 않는다
}
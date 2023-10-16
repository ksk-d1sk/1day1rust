// #클로저 Copy의 조건#
// : 비 move closure
// : : 캡처된 값들이 불변 참조여야함
// : move closure
// : : 캡처된 값들이 Copy trait를 구현하고 있어야함

fn main() {
    let y = 10;
    let add_y = |x| x + y;
    let copy_of_add_y = add_y;  // Copy 됨
    assert_eq!(add_y(copy_of_add_y(22)), 42);

    let mut x = 0;
    let mut add_to_x = |n| {x += n; x};
    let mut copy_of_add_to_x = add_to_x;  // 이동 됨
    // assert_eq!(add_to_x(1), 1);  // 클로저가 이동되어 사용을 시도하면 오류가 발생함
    assert_eq!(copy_of_add_to_x(1), 1);

    let mut greeting = String::from("Hello, ");
    let greet = move |name| {
        greeting.push_str(name);
        println!("{}!", greeting);
    };
    greet.clone()("World");
    greet.clone()("Rust");
}

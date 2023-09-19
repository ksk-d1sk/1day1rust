struct A {
    text: &'static str,
}

// Copy 트레이트가 구현된 타입은 값이 스택에 저장된다.
#[derive(Clone, Copy)]
struct B {
    text: &'static str,
}

/*
// Copy 타입을 구현하는 구조체는 동적 크기 타입을 포함할 수 없다.
#[derive(Clone, Copy)]
struct C {
    text: String,
}
*/

fn main() {
    /*
    let a = A { text: "Hello" };
    let b = a;                   // 소유권 이동

    println!("{}", a.text);      // error! b로 값의 복사가 아닌 소유권이 이동하여 a를 사용하지 못하게 된다.
    */

    let a = B { text: "Hello" };
    let b = a;                   // 값 복사

    println!("{}", a.text);      // Copy 트레이트를 구현하게 되면 할당 또는 함수에 전달 시 소유권 이동이 아닌 값 복사가 발생한다.

    // i32를 포함한 기본 정수, 실수형 데이터 타입들은 Copy 트레이트를 구현한다.
    let x = 256_i32;
    let y = x;                   // 소유권 이동이 아닌 복사가 된다.
    println!("{} {}", x, y);
}

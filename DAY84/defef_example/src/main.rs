use std::ops::{Deref, DerefMut};

struct A<T> {
    var: T,
}

impl<T> Deref for A<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.var
    }
}

struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

fn main() {
    let string = String::from("Hello, world!");
    let slice: &str = &string;          // 이 코드에 대한 실제 내부 동작은 아래와 같다
    let slice: &str = &(*string);       // *string = str, &(*string) = &str

    let v = vec![65, 66, 67, 68, 69];
    print_u8_str(&v);                   // 이 코드에 대한 실제 내부 동작은 아래와 같다
    print_u8_str(&(*v));                // *v = [u8], &(*v) = &[u8]

    let a = A { var: 123_i32 };
    let b = *a;
    assert_eq!(b, 123i32);

    let mut s = Selector { elements: vec!['x', 'y', 'z'],
                           current: 2 };
    assert_eq!(*s, 'z');

    // Deref 강제 변환이 적용되어 char 타입의 메서드를 Select에 대고 바로 사용할 수 있다
    assert!(s.is_alphabetic());

    *s = 'w';

    assert_eq!(s.elements, ['x', 'y', 'w']);
}

fn print_u8_str(slice: &[u8]) {
    for c in slice {
        print!("{}", *c as char);
    }
    println!();
}

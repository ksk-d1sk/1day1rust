use std::ops::{Deref, DerefMut};
use std::fmt::Display;

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
    let s = Selector { elements: vec!["good",  "bad", "ugly"],
                       current: 2 };

    show_it(&s);                  // ok
//  show_it_generic(&s);          // error
    show_it_generic(&s as &str);  // ok
    show_it_generic(&*s);         // ok

    // 이해가 안가요 엉엉ㅠㅠㅠㅠㅠㅠㅠㅠ
}

fn show_it(thing: &str) {
    println!("{}", thing);
}

fn show_it_generic<T: Display>(thing: T) {
    println!("{}", thing);
}
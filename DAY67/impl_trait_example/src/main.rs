use std::fmt::Display;
use std::iter;
use std::vec::IntoIter;

// Bad
fn cyclical_zip1(v: Vec<u8>, u: Vec<u8>) -> iter::Cycle<iter::Chain<IntoIter<u8>, IntoIter<u8>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// Not bad
fn  cyclical_zip2(v: Vec<u8>, u: Vec<u8>) -> Box<dyn Iterator<Item = u8>> {
    Box::new(v.into_iter().chain(u.into_iter()).cycle())
}

// Good
fn cyclical_zip3(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item = u8> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn print1<T>(val: T)
where
    T: Display,
{
    println!("{val}");
}

fn print2(val: impl Display) {
    println!("{val}");
}

fn main() {
    print1("Hello, world!");
    print2("Hello, rust!");
}

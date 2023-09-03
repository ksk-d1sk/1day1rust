use std::ops::Add;

trait Greet {
    const GREETING: &'static str = "Hello";
    fn greet(&self) -> String;
}

trait Float {
    const ZERO: Self;
    const ONE: Self;
}

impl Float for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl Float for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

fn add_one<T>(value: T) -> T
where
    T: Float + Add<Output = T>
{
    value + T::ONE
}

fn fib<T>(n: usize) -> T
where
    T: Float + Add<Output = T>
{
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        _ => fib::<T>(n - 1) + fib::<T>(n - 2)
    }
}

fn main() {
    assert_eq!(add_one(4.5), 5.5);
    assert_eq!(fib::<f64>(7), 13.0);
}

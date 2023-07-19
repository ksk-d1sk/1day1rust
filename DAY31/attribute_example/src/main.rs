#![allow(non_snake_case)]

#[allow(non_camel_case_types)]
pub struct git_revspec {}

#[cfg(target_os = "android")]
mod mobile;

#[inline]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[inline(always)]
pub fn mul(a: u32, b: u32) -> u32 {
    a * b
}

#[inline(never)]
pub fn div(a: u32, b: u32) -> u32 {
    a / b
}

pub fn Hello_worlD() {
    println!("Hello, world!");
}

#[test]     // cargo test 또는 cargo t 를 통해 단위테스트 실행
fn math_works() {
    let x: i32 = 1;
    assert!(x.is_positive());
    assert_eq!(x + 1, 2);
}

#[test]
#[allow(unconditional_panic, unused_must_use)]
#[should_panic(expected="attempt to divide by zero")]
fn test_divide_by_zero_error() {
    1 / 0;
}

#[test]
fn trig_works() {
    use std::f64::consts::PI;
    assert!(roughly_equal(PI.sin(), 0.0));
}

fn roughly_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

#[cfg(test)]
mod tests {
    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert!(roughly_equal(PI.sin(), 0.0));
    }
    
    fn roughly_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }
}

fn main() {
    debug_assert!(!true);
    debug_assert_eq!(true, false);
}

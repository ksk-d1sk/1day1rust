fn main() {
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);

    f(&WORTH_POINTING_AT);

    let x = 10;
    g(&x);
}

static mut STASH: &i32 = &128;
static WORTH_POINTING_AT: i32 = 1000;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn g<'a>(p: &'a i32) {}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

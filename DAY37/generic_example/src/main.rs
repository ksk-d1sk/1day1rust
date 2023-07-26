pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

impl Queue<f64> {
    fn get_pi(&self) -> f64 {
        3.1415926535897
    }
}

struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least    { least    = &slice[i] }
        if slice[i] > *greatest { greatest = &slice[i] }
    }

    Extrema { greatest, least }
}

struct Polynomial<const N: usize> {
    coefficients: [f64; N],
}

impl<const N: usize> Polynomial<N> {
    fn new(coefficients: [f64; N]) -> Self {
        Polynomial { coefficients }
    }

    fn eval(&self, x: f64) -> f64 {
        let mut sum = 0.;
        for i in (0..N).rev() {
            sum = self.coefficients[i] + x * sum;
        }

        sum
    }
}

fn main() {
    let f64_q: Queue<f64> = Queue::new();
    println!("{}", f64_q.get_pi());

    let i32_q: Queue<i32> = Queue::new();
    // println!("{}", i32_q.get_pi());      // error!

    let mut q = Queue::<char>::new();     // turbofish

    let mut q = Queue::new();
    let mut r = Queue::new();

    q.push("CAD");     // Queue<&'static str>
    r.push(0.74);      // Queue<f64>

    q.push("BTC");
    r.push(13764.0);

    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);

    assert_eq!(e.least,    &-3);
    assert_eq!(e.greatest, &48);
}

use num::Complex;
use rand::random;
use std::iter::{from_fn, successors};

fn main() {
    let lengths: Vec<f64> =
        from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(25)
        .collect();

    println!("{:?}", lengths);

    assert_eq!(fibonacci().take(8).collect::<Vec<_>>(), 
               [1, 1, 2, 3, 5, 8, 13, 21]);

    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));
    println!("{}", outer);
    println!("{}", inner);

    let step: Vec<i32> = (1..10).step_by(2).collect();
    assert_eq!(step, [1, 3, 5, 7, 9]);
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let zero = Complex { re: 0., im: 0. };
    successors(Some(zero), |&z| Some(z * z + c))
        .take(limit)
        .enumerate()
        .find(|(_, z)| z.norm_sqr() > 4.0)
        .map(|(i, _)| i)
}

fn fibonacci() -> impl Iterator<Item = usize> {
    let mut state = (0, 1);
    from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}

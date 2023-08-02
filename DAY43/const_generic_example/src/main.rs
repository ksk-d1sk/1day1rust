use std::f64::consts::FRAC_PI_2;

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

struct LumpOfReferences<'a, T, const N: usize> {
    the_lump: [&'a T; N]
}

fn main() {
    let sine_poly = Polynomial::new(
        [0.0, 1.0, 0.0, -1.0 / 6.0, 0.0, 1.0 / 120.0]
    );

    assert_eq!(sine_poly.eval(0.0), 0.0);
    assert!((sine_poly.eval(FRAC_PI_2) - 1.0).abs() < 0.005);
}

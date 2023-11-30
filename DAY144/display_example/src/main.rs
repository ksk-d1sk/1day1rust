use std::fmt;

struct Complex {
    re: f64,
    im: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Complex { re, im } = self;
        if dest.alternate() {
            let abs = f64::sqrt(re * re + im * im);
            let angle = f64::atan2(*im, *re) / std::f64::consts::PI * 180.0;
            write!(dest, "{} ∠ {}°", abs, angle)
        } else {
            let im_sign = if *im < 0.0 { '-' } else { '+' };
            write!(dest, "{} {} {}i", re, im_sign, f64::abs(*im))
        }
    }
}

fn main() {
    let ninety = Complex { re: 0.0, im: 2.0 };
    println!("{}"  , ninety);
    println!("{:#}", ninety);
}

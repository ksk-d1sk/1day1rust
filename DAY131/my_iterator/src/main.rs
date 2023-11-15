struct I32Range {
    start: i32,
    end: i32,
}

impl Iterator for I32Range {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            None
        } else {
            let result = Some(self.start);
            self.start += 1;
            result
        }
    }
}

fn main() {
    let mut pi = 0.0;
    let mut  numerator = 1.0;

    for k in (I32Range { start: 0, end: 14 }) {
        pi += numerator / (2 * k + 1) as f64;
        numerator /= -3.0;
    }

    pi *= f64::sqrt(12.0);

    assert_eq!(pi as f32, std::f32::consts::PI);
}

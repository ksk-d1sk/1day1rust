use std::borrow::Cow;

// Rust 공식 Cow 소개 Example

fn abs_all(input: &mut Cow<'_, [i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }
}

fn main() {
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);
    println!("{:?}", input);

    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);
    println!("{:?}", input);

    let vector = vec![-1, 0, 1];
    let mut input = Cow::from(vector);
    abs_all(&mut input);
    println!("{:?}", input);
}

use num::Num;
use std::ops::{AddAssign, Mul};

fn dot1(v1: &[i64], v2: &[i64]) -> i64 {
    let mut total = 0;
    for i in 0..v1.len() {
        total += v1[i] * v2[i];
    }

    total
}

fn dot2<N>(v1: &[N], v2: &[N]) -> N
where
    N: AddAssign + Mul<Output = N> + Default + Copy,
{
    let mut total: N = N::default();
    for i in 0..v1.len() {
        total += v1[i] * v2[i];
    }

    total
}

fn dot3<N>(v1: &[N], v2: &[N]) -> N
where
    N: Num + AddAssign + Copy,
{
    let mut total: N = N::zero();
    for i in 0..v1.len() {
        total += v1[i] * v2[i];
    }

    total
}

fn main() {
    assert_eq!(dot1(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot2(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);

    assert_eq!(dot2(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
    assert_eq!(dot3(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}

use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    // iterator 아이템이 Ord trait를 구현하고 있어야 한다.
    assert_eq!([-2, 0, 1, 0, -1, -5].into_iter().max(), Some(1));
    assert_eq!([-2, 0, 1, 0, -1, -5].into_iter().min(), Some(-5));

    let v: Vec<i32> = Vec::new();
    assert_eq!(v.iter().max(), None);
    assert_eq!(v.iter().min(), None);

    // f64와 f32는 PartialOrd만 구현하므로 max_by, min_by를 사용하여야 한다.
    let numbers = [1.0, 4.0, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
    assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));

    // max_by_key, min_by_key
    let mut population = HashMap::new();
    population.insert("Portland",  583_776);
    population.insert("Fossil",        449);
    population.insert("Greenhorn",       2);
    population.insert("Boring",      7_762);
    population.insert("The Dalles", 15_340);

    assert_eq!(population.iter().max_by_key(|&(_name, pop)| pop),
               Some((&"Portland", &583_776)));
    assert_eq!(population.iter().min_by_key(|&(_name, pop)| pop),
               Some((&"Greenhorn", &2)));
}

fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
    lhs.partial_cmp(&rhs).unwrap()
}
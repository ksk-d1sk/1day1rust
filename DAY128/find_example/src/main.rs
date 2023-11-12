use std::collections::HashMap;

fn main() {
    let mut population = HashMap::new();
    population.insert("Portland",  583_776);
    population.insert("Fossil",        449);
    population.insert("Greenhorn",       2);
    population.insert("Boring",      7_762);
    population.insert("The Dalles", 15_340);

    assert_eq!(population.iter().find(|&(_name, &pop)| pop > 1_000_000), None);
    assert_eq!(population.iter().find(|&(_name, &pop)| pop > 500_000)  , Some((&"Portland", &583_776)));
}

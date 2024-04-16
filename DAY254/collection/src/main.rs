use std::collections::HashMap;

fn main() {
    let mut m: HashMap<String, i32> = HashMap::new();

    let key = String::from("foo");
    let value = match m.get(&key) {
        Some(v) => v,
        None => {
            m.insert(key.clone(), 42);
            &m[&key]
        }
    };
    println!("{}", value);

    let key = String::from("bar");
    let value = m.entry(key).or_insert(7);
    println!("{}", value);
}

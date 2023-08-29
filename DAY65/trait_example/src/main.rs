fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut res = Vec::new();
    for value in iter {
        res.push(value);
    }
    res
}

fn dump1<I>(iter: I)
where
    I: Iterator,
    I::Item: std::fmt::Debug,
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

fn dump2<I>(iter: I)
where
    I: Iterator<Item = String>,
{
    for (index, s) in iter.enumerate() {
        println!("{}: {:?}", index, s);
    }
}

fn main() {
    println!("Hello, world!");
}

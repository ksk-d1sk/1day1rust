fn main() {
    println!("Hello, world!");
}
ause std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut arr1: Vec<_> = (0..n).map(|_| next!(u32)).collect();
    let mut arr2 = vec![0; n];

    for i in 1..n {
        if arr1[i] < arr1[i - 1] {
            arr1[i] = arr1[i - 1];
            arr2[i] = arr2[i - 1] + 1;
        }
    }

    print!("{}", arr2.iter().max().unwrap());
}
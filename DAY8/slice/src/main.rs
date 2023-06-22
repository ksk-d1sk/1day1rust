fn main() {
    let v = vec![0.0, 0.707, 1.0, 0.707];
    let a =     [0.0, 0.707, 1.0, 0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    print(sa);
    print(sv);

    print(&v[0..2]);
    print(&a[2..]);
    print(&sv[1..3]);
}


fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
    println!();
}
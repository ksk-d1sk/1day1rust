fn main() {
    assert_eq!(5_f32.sqrt() * 5_f32.sqrt(), 5.);
    assert_eq!((-1.01_f64).floor(), -2.0);

    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));

    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };
    println!("raw points as {}",  points_at);

    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10_000];
    for i in 2..100 {
        if sieve[i] {
            let mut  j = i * i;
            while j < 10_000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort_unstable();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}
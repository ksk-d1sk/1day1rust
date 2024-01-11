fn read_i32() -> i32 {
    let mut resp_s = String::new();
    std::io::stdin().read_line(&mut resp_s).unwrap();
    resp_s.trim().parse().unwrap()
}

fn main() {
    let mut i = 0;
    let a = loop {
        i += 1;

        println!("? A {i}");

        let resp = read_i32();

        if resp == 1 {
            break i;
        }
    };

    i = 0;
    let b = loop {
        i += 1;

        println!("? B {i}");

        let resp = read_i32();

        if resp == 1 {
            break i;
        }
    };

    print!("! {}", a + b);
}
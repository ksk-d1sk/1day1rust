fn triangle1(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn triangle2(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

fn main() {
    // 둘 다 동일한 기능을 하는 함수이다
    println!("{}", triangle1(10));
    println!("{}", triangle2(10));

    println!("There's:");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

    // 둘 다 같은 기능을 수행한다
    // : code 1
    for element in &v {
        println!("{}", element);
    }

    // : code 2
    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }
}

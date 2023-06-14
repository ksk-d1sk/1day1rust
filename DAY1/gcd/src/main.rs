use std::env;            // 실행 환경과 상호 작용하는 데 유용한 라이브러리

fn main() {
    let numbers: Vec<u64> = env::args()
        .skip(1)
        .map(|arg| arg.parse().expect("error parsing argument"))
        .collect();

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

/// # 최대공약수 함수
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);         // 인수가 false이면 오류 발생

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m %= n;
    }

    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    let a      = 2 * 3 * 5   * 11      * 17     ;
    let b      =     3 *   7 * 11 * 13      * 19;
    let answer =     3       * 11               ;
    assert_eq!(gcd(a, b), answer);
}
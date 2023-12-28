use std::fs::File;
use std::io::{Write, Read};

fn main() {
    let execute = _mk_counter_range;
    execute();
}

fn _mk_counter_range() {
    let mut f_counter = File::open("counter.txt").expect("_mk_counter() 함수를 먼저 실행하세요");
    let mut f_counter_range = File::create("counter_range.txt").expect("파일 생성 실패");
    let mut buf = String::new();
    f_counter.read_to_string(&mut buf).unwrap();

    let v: Vec<usize> = buf.lines().flat_map(str::parse).collect();
    let mut i = 0;

    while i < v.len() {
        let front = v[i];
        let back = loop {
            i += 1;
            if !v.get(i).is_some_and(|x| x - v[i - 1] == 1) { break v[i - 1]; }
        };

        if let Err(msg) = writeln!(f_counter_range, "({}, {})", front, back) {
            eprintln!("{msg}");
        }
    }
}

fn _mk_counter() {
    let mut f_counter = File::create("counter.txt").expect("파일 생성 실패");
    const N: usize = 1_000_000_000;

    println!("에라토스테네스의 체 생성중...");
    let min_factor_list = _get_min_factor_list(N);
    println!("생성 완료!!");

    let mut even_count = 0; // 짝수 카운트
    let mut odd_count = 0;  // 홀수 카운트

    for i in 1..N {
        let mut k = i;
        let mut count = 0;

        while 1 < k {
            count += 1;
            k /= min_factor_list[k];
        }

        if count & 1 == 1 {
            odd_count += 1;
        } else {
            even_count += 1;
        }

        if even_count > odd_count {
            if let Err(msg) = writeln!(f_counter, "{i}") {
                eprintln!("{msg}");
            }
        }
    }
}

fn _get_min_factor_list(n: usize) -> Vec<usize> {
    let mut min_factor_list: Vec<usize> = (0..=n).collect();
    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 2..=sqrt_n {
        if min_factor_list[i] == i {
            for j in ((i * i)..=n).step_by(i) {
                if min_factor_list[j] == j {
                    min_factor_list[j] = i;
                }
            }
        }
    }

    min_factor_list
}
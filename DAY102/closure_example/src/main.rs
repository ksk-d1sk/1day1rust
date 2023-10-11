fn main() {
    let my_str = "Hello".to_string();
    let f = || drop(my_str);

    f();
//  f();// error! f는 타입이 FnOnce이므로 f를 처음 호출 시 소유권이 이동된다

    let v = vec![1, 2, 3 , 4, 5];
    let print_vec = || {
        for elem in v {  // v의 소유권을 차용하여 print_vec의 타입은 FnOnce가 된다
            print!("{} ", elem);
        }
        println!();
    };

    print_vec();
//  print_vec();    // error!

    let mut i = 0;
    let mut incr = || {
        i += 1;    // 값이 변경되므로 i를 mut reference로 가져온다
        println!("Ding! i is now: {}", i);
    };

    incr();
    incr();
    incr();
}
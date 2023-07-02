fn main() {
    /* if 표현식 */
    let condition1 = false;
    let condition2 = true;

    let message = if condition1 {
        "Hello!"
    } else if condition2 {
        "Rust!"
    } else {
        "Programming!"
    };

    println!("{}", message);

    // let _temp = if true { "Hello" } else { "Rust" };   // OK
    // let _temp = if true { "Hello" } else { 123 };      // error (if 표현식의 모든 블록은 같은 타입의 값을 산출해야 한다)
    // let _temp = if true { "Rust" };                    // error (if 표현식은 모든 경우를 처리해야 한다)

    /* if let */
    let fruit_list = vec!["Apple", "Banana", "Peach"];

    if let Some(fruit) = fruit_list.get(0) {
        println!("{}", fruit);
    } else {
        println!("index out of bounds");
    }

    if let Some(fruit) = fruit_list.get(4) {
        println!("{}", fruit);
    } else {
        println!("index out of bounds");
    }

    /* loop */

    // C언어의 for문
    // for (int i = 0; i < 20; i++) {
    //     printf("%d ", i);
    // }

    // Rust의 for문
    for i in 0..20 {
        print!("{} ", i);
    }
    println!();


    let string = vec!["DJMAX", "is", "god", "game"];
    for s in string {
        print!("{} ", s);
    }
    // println!("{:?}", string);      // error (for 루프로 값을 반복 처리할 떄는 러스트의 이동 의미론에 따라 값이 소비된다)
    println!();


    let string = vec!["Arcaea", "is", "god", "game"];
    for s in &string {
        print!("{} ", s);
    }
    println!("\n{:?}", string);         // OK


    let mut string = vec!["Ramen".to_string(), "is".to_string(), "god".to_string()];
    for s in &mut string {
        s.push_str("#ΦωΦ");
    }
    println!("{:?}", string);         // OK


    let mut string = r"
Trrrrrr
Skibidi, dom, dom, dom, yes, yes
Skibidi, dabudi, dip, dip
Shtibidi, dom, dom, dom, yes, yes
Shtibidi, dabudi, dip, dip
We ain't here to hurt nobody
answer: Skibidi, skibidi, skibidi
Wanna see you work your body
Shtibidi, shtibidi, shtibidi
    ".lines();

    let answer = loop {
        if let Some(line) = string.next() {
            if line.starts_with("answer: ") {
                break line;
            }
        } else {
            break "answer: no shtibidi";
        }
    };

    println!("{}", answer);


    let mut num_list = [8, 5, 12, 54, 64, 72, 225].iter();
    let sqrt = 'outer: loop {
        if let Some(&n) = num_list.next() {
            for i in 1.. {
                let square = i * i;
                if square == n {
                    break 'outer i;
                }
                if square > n {
                    break;
                }
            }
        } else {
            break -1;
        }
    };
    println!("{}", sqrt);
}

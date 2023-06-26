#[derive(Clone, Copy)]
struct Label { number: u32 }

fn main() {
    let string1 = "somnambulance".to_string();
    let string2 = string1;

    let num1 = 36;
    let num2 = num1;

    print!("{} {}", string2, num2);

    let l = Label { number: 3 };
    print(l);
    println!("My label number is: {}", l.number);
}

fn print(l: Label) {
    println!("STAMP: {}", l.number);
}

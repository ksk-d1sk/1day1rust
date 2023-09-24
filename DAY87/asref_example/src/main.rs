fn greet<T: AsRef<str>>(name: T) {
    println!("hello {}!", name.as_ref());
}
 
 fn main() {
    let s = "John";
    greet(s);

    let s = "Park".to_string();
    greet(s);
}

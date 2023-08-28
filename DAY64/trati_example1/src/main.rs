trait StringSet {
    fn new() -> Self
    where
        Self: Sized;

    fn from_slice(strings: &[&str]) -> Self
    where
        Self: Sized;

    fn  contains(&self, string: &str) -> bool;

    fn add(&mut self, string: &str);
}

fn unknown_words<S: StringSet>(document: &[String], wordlist: &S) -> S {
    let mut unknowns = S::new();

    for word in document {
        if !wordlist.contains(word) {
            unknowns.add(word);
        }
    }

    unknowns
}

trait Visible {
    fn draw(&self);
}

trait HasPistol {
    fn draw(&self);
}

struct Outlaw {
    name: &'static str,
}

impl Outlaw {
    fn new(name: &'static str) -> Outlaw {
        Outlaw { name }
    }
}

impl Visible for Outlaw {
    fn draw(&self) {
        println!("{}: 슥삭슥삭~", self.name);
    }
}

impl HasPistol for Outlaw {
    fn draw(&self) {
        println!("{}: 빵야빵야!", self.name);
    }
}

fn main() {
    // 모두 같은 hello String
    "hello".to_string();
    str::to_string("hello");
    ToString::to_string("hello");
    <str as ToString>::to_string("hello");

    let outlaw = Outlaw::new("센세");

//  outlaw.draw();             // error: 어떤 trait의 메서드인지 알 수 없다
    Visible::draw(&outlaw);    // OK!
    HasPistol::draw(&outlaw);  // OK!

    let zero = 0;              // `i8`이나 `u8` 등이 될 수 있다.
//  zero.abs();                // error: 모호한 수치 타입에 `abs` 메서드를 호출할 수 없다.
    i32::abs(zero);            // OK!

    let numbers: Vec<u32> = "2 3 5 7 11 13 17 메롱 19 23 29 31 37"
        .split_ascii_whitespace()
        .flat_map(str::parse)
        .collect();

    println!("{:?}", numbers);
}

use std::cell::RefCell;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = get!(usize, usize);
    let prime = RefCell::new(vec![true; n + 1]);
    let mut removed = Vec::with_capacity(n);

    for i in (2..=n).filter(|&i| prime.borrow()[i]) {
        for j in (1..).take_while(|j| i * j <= n).filter_map(|j| prime.borrow()[i * j].then_some(i * j)) {
            (*prime.borrow_mut())[j] = false;
            removed.push(j);
        }
    }

    print!("{}", removed[k - 1]);
}
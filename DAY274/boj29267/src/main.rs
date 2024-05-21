use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = next!(u16, u16);
    let mut ammo = 0;
    let mut save = 0;

    for cmd in (0..n).map(|_| next!()) {
        match cmd {
            "save"  => save = ammo,
            "load"  => ammo = save,
            "shoot" => ammo -= 1,
            "ammo"  => ammo += k,
            _       => unreachable!(),
        }
        let _ = writeln!(output, "{ammo}");
    }

    print!("{output}");
}
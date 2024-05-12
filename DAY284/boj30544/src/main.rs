use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let [mut hh, mut mm] = next!().split(':').flat_map(str::parse::<i32>).collect::<Vec<_>>()[..] else { panic!() };
    let mut count_down = next!(i32);

    if mm == 0 {
        count_down -= hh;
    } else if mm % 15 == 0 {
        count_down -= 1;
    }

    while count_down > 0 {
        mm = (mm / 15 + 1) % 4 * 15;
        if mm == 0 {
            hh = hh % 12 + 1;
            count_down -= hh;
        } else {
            count_down -= 1;
        }
    }

    print!("{:02}:{:02}", hh, mm);
}
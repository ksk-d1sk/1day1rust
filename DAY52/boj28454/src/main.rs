use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let now_date: Vec<u16> = get!().split('-').flat_map(str::parse).collect();
    let mut count = 0;

    for date in input.skip(1).map(|e| e.split('-').flat_map(str::parse).collect::<Vec<u16>>()) {
        let mut check = true;
        for i in 0..3 {
            if date[i] > now_date[i] {
                break;
            } else if check && date[i] < now_date[i] {
                check = false;
            }
        }

        if check {
            count += 1;
        }
    }

    print!("{count}");
}

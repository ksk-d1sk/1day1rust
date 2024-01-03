use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
    }

    let time1: Vec<i8> = get!().split(':').flat_map(str::parse).collect();
    let time2: Vec<i8> = get!().split(':').flat_map(str::parse).collect();
    let (mut hh, mut mm, mut ss) = (
        time2[0] - time1[0],
        time2[1] - time1[1],
        time2[2] - time1[2],
    );

    if hh == 0 && mm == 0 && ss == 0 {
        hh = 24;
    } else {
        if ss < 0 { ss += 60; mm -= 1 }
        if mm < 0 { mm += 60; hh -= 1 }
        if hh < 0 { hh += 24 }
    }

    print!("{:02}:{:02}:{:02}", hh, mm, ss);
}
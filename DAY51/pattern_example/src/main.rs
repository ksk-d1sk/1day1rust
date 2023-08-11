fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;

    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal)     => "at the origin",
        (_, Equal)         => "on the x axis",
        (Equal, _)         => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater)    => "in the second quadrant",
        _                  => "somewhere else",
    }
}

fn hsl_rgb(hsl: [u8; 3]) -> [u8; 3] {
    match hsl {
        [_, _, 0]   => [0, 0, 0],
        [_, _, 255] => [255, 255, 255],
        _           => [31, 30, 51],
    }
}

fn greet_people(names: &[&str]) {
    match names {
        []         => { println!("Hello, nobody.")                       },
        [a]        => { println!("Hello, {}.", a)                        },
        [a, b]     => { println!("Hello, {} and {}.", a, b)              },
        [a, .., b] => { println!("Hello, everyone from {} to {}.", a, b) },
    }
}

fn main() {
    println!("Hello, world!");
}

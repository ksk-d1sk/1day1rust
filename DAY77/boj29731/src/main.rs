use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for line in buf.lines().skip(1) {
        match line {
            "Never gonna give you up"               |
            "Never gonna let you down"              |
            "Never gonna run around and desert you" |
            "Never gonna make you cry"              |
            "Never gonna say goodbye"               |
            "Never gonna tell a lie and hurt you"   |
            "Never gonna stop" => continue,
            _ => {
                print!("Yes");
                return;
            },
        }
    }

    print!("No");
}
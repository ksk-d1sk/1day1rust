use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut king = parse_to_tuple(next!());
    let mut stone = parse_to_tuple(next!());
    let n = next!(u8);

    for cmd in (0..n).map(|_| next!()) {
        let (king_next, stone_next) = match cmd {
            "R"  => ((king.0 + 1, king.1), (stone.0 + 1, stone.1)),
            "L"  => ((king.0 - 1, king.1), (stone.0 - 1, stone.1)),
            "B"  => ((king.0, king.1 - 1), (stone.0, stone.1 - 1)),
            "T"  => ((king.0, king.1 + 1), (stone.0, stone.1 + 1)),
            "RT" => ((king.0 + 1, king.1 + 1), (stone.0 + 1, stone.1 + 1)),
            "LT" => ((king.0 - 1, king.1 + 1), (stone.0 - 1, stone.1 + 1)),
            "RB" => ((king.0 + 1, king.1 - 1), (stone.0 + 1, stone.1 - 1)),
            "LB" => ((king.0 - 1, king.1 - 1), (stone.0 - 1, stone.1 - 1)),
            _ => unreachable!(),
        };

        if 1 <= king_next.0 && king_next.0 <= 8 && 1 <= king_next.1 && king_next.1 <= 8 {
            if king_next == stone {
                if 1 <= stone_next.0 && stone_next.0 <= 8 && 1 <= stone_next.1 && stone_next.1 <= 8 {
                    king = king_next;
                    stone = stone_next;
                }
            } else {
                king = king_next;
            }
        }
    }

    print!("{}\n{}", parse_to_coord(king), parse_to_coord(stone));
}

fn parse_to_tuple(s: &str) -> (u8, u8) {
    let s = s.as_bytes();
    ((s[0] - b'A' + 1) as u8, (s[1] - b'0') as u8)
}

fn parse_to_coord(t: (u8, u8)) -> String {
    format!("{}{}", (t.0 - 1 + b'A') as char, (t.1 + b'0') as char)
}
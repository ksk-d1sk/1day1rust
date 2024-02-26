use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let big_num = [
"0000
0  0
0  0
0  0
0000

",
"   1
   1
   1
   1
   1

",
"2222
   2
2222
2
2222

",
"3333
   3
3333
   3
3333

",
"4  4
4  4
4444
   4
   4

",
"5555
5
5555
   5
5555

",
"6666
6
6666
6  6
6666

",
"7777
   7
   7
   7
   7

",
"8888
8  8
8888
8  8
8888

",
"9999
9  9
9999
   9
   9

",
    ];

    let s = next!();
    for i in s.bytes().map(|b| (b - b'0') as usize) {
        output.push_str(big_num[i]);
    }

    print!("{output}");
}
use std::collections::HashMap;
use std::io::*;

const ADJ: [[(usize, u8); 5]; 6] = [
    [(3, 2), (5, 3), (2, 0), (4, 1), (3, 2)],
    [(3, 0), (4, 3), (2, 2), (5, 1), (3, 0)],
    [(0, 2), (5, 2), (1, 2), (4, 2), (0, 2)],
    [(1, 0), (5, 0), (0, 0), (4, 0), (1, 0)],
    [(3, 3), (0, 3), (2, 3), (1, 1), (3, 3)],
    [(3, 1), (1, 3), (2, 1), (0, 1), (3, 1)],
];

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(u8);
    let trans: HashMap<_, _> = HashMap::from([
        (b'U', 0), (b'D', 1), (b'F', 2),
        (b'B', 3), (b'L', 4), (b'R', 5),
    ]);

    for _ in 0..t {
        let n = next!(u16);
        let mut cube = [
            // 윗 면 (white)
            [['w', 'w', 'w'],
             ['w', 'w', 'w'],
             ['w', 'w', 'w']],

            // 아랫 면 (yellow)
            [['y', 'y', 'y'],
             ['y', 'y', 'y'],
             ['y', 'y', 'y']],

            // 앞 면 (red)
            [['r', 'r', 'r'],
             ['r', 'r', 'r'],
             ['r', 'r', 'r']],

            // 뒷 면 (orange)
            [['o', 'o', 'o'],
             ['o', 'o', 'o'],
             ['o', 'o', 'o']],

            // 왼쪽 면 (green)
            [['g', 'g', 'g'],
             ['g', 'g', 'g'],
             ['g', 'g', 'g']],

            // 오른쪽 면 (blue)
            [['b', 'b', 'b'],
             ['b', 'b', 'b'],
             ['b', 'b', 'b']],
        ];

        for cmd in (0..n).map(|_| next!().as_bytes()) {
            let i = trans[&cmd[0]];
            if cmd[1] == b'+' {
                spin_a(&mut cube[i]);
                spin_b(i, &mut cube);
            } else {
                r_spin_a(&mut cube[i]);
                r_spin_b(i, &mut cube);
            }
        }

        for v in cube[0] {
            for color in v {
                output.push(color);
            }
            output.push('\n');
        }
    }

    print!("{output}");
}

fn spin_a(v: &mut [[char; 3]; 3]) {
    (
        v[0][2],
        v[1][2],
        v[2][2],
        v[2][1],
        v[2][0],
        v[1][0],
        v[0][0],
        v[0][1],
    ) = (
        v[0][0],
        v[0][1],
        v[0][2],
        v[1][2],
        v[2][2],
        v[2][1],
        v[2][0],
        v[1][0],
    );
}

fn r_spin_a(v: &mut [[char; 3]; 3]) {
    (
        v[2][0],
        v[1][0],
        v[0][0],
        v[0][1],
        v[0][2],
        v[1][2],
        v[2][2],
        v[2][1],
    ) = (
        v[0][0],
        v[0][1],
        v[0][2],
        v[1][2],
        v[2][2],
        v[2][1],
        v[2][0],
        v[1][0],
    );
}

fn spin_b(i: usize, cube: &mut [[[char; 3]; 3]; 6]) {
    let (mut t1, mut t2, mut t3) = (' ', ' ', ' ');
    for (j, x) in ADJ[i] {
        match x {
            0 => {
                (
                    t1, t2, t3,
                    cube[j][0][0], cube[j][0][1], cube[j][0][2],
                ) = (
                    cube[j][0][0], cube[j][0][1], cube[j][0][2],
                    t1, t2, t3,
                );
            }
            1 => {
                (
                    t1, t2, t3,
                    cube[j][0][2], cube[j][1][2], cube[j][2][2],
                ) = (
                    cube[j][0][2], cube[j][1][2], cube[j][2][2],
                    t1, t2, t3,
                );
            }
            2 => {
                (
                    t1, t2, t3,
                    cube[j][2][2], cube[j][2][1], cube[j][2][0],
                ) = (
                    cube[j][2][2], cube[j][2][1], cube[j][2][0],
                    t1, t2, t3,
                );
            }
            3 => {
                (
                    t1, t2, t3,
                    cube[j][2][0], cube[j][1][0], cube[j][0][0],
                ) = (
                    cube[j][2][0], cube[j][1][0], cube[j][0][0],
                    t1, t2, t3,
                );
            }
            _ => unreachable!(),
        }
    }
}

fn r_spin_b(i: usize, cube: &mut [[[char; 3]; 3]; 6]) {
    let (mut t1, mut t2, mut t3) = (' ', ' ', ' ');
    for (j, x) in ADJ[i].iter().rev().copied() {
        match x {
            0 => {
                (
                    t1, t2, t3,
                    cube[j][0][0], cube[j][0][1], cube[j][0][2],
                ) = (
                    cube[j][0][0], cube[j][0][1], cube[j][0][2],
                    t1, t2, t3,
                );
            }
            1 => {
                (
                    t1, t2, t3,
                    cube[j][0][2], cube[j][1][2], cube[j][2][2],
                ) = (
                    cube[j][0][2], cube[j][1][2], cube[j][2][2],
                    t1, t2, t3,
                );
            }
            2 => {
                (
                    t1, t2, t3,
                    cube[j][2][2], cube[j][2][1], cube[j][2][0],
                ) = (
                    cube[j][2][2], cube[j][2][1], cube[j][2][0],
                    t1, t2, t3,
                );
            }
            3 => {
                (
                    t1, t2, t3,
                    cube[j][2][0], cube[j][1][0], cube[j][0][0],
                ) = (
                    cube[j][2][0], cube[j][1][0], cube[j][0][0],
                    t1, t2, t3,
                );
            }
            _ => unreachable!(),
        }
    }
}
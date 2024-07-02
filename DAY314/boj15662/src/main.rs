use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(usize);
    let mut gear = Vec::from_iter((0..t).map(|_| next!().bytes().map(|i| i - 48).collect::<Vec<_>>()));
    let k = next!(u16);

    for (n, ns) in (0..k).map(|_| next!(usize, i8)) {
        let n = n - 1;

        let mut md = ns;
        for i in n..t {
            let check = i + 1 == t || gear[i][2] == gear[i + 1][6];

            if i != n {
                if md == 1 {
                    r_spin(&mut gear[i]);
                } else {
                    l_spin(&mut gear[i]);
                }
            }

            if check {
                break;
            }

            md = -md;
        }

        let mut md = ns;
        for i in (0..=n).rev() {
            let check = i == 0 || gear[i][6] == gear[i - 1][2];

            if i != n {
                if md == 1 {
                    r_spin(&mut gear[i]);
                } else {
                    l_spin(&mut gear[i]);
                }
            }

            if check {
                break;
            }

            md = -md;
        }

        if ns == 1 {
            r_spin(&mut gear[n]);
        } else {
            l_spin(&mut gear[n]);
        }
    }

    print!("{}", gear.iter().filter(|v| v[0] == 1).count());
}

fn r_spin(v: &mut [u8]) {
    (
        v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]
    ) = (
        v[7], v[0], v[1], v[2], v[3], v[4], v[5], v[6]
    );
}

fn l_spin(v: &mut [u8]) {
    (
        v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]
    ) = (
        v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[0]
    );
}
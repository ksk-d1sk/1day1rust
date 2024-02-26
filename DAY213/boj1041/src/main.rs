use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u64);
    let dice: Vec<_> = (0..6).map(|_| next!(u64)).collect();

    print!(
        "{}",
        if n == 1 { min_side_5(&dice) }
        else { solve(n, &dice) }
    );
}

fn solve(n: u64, dice: &[u64]) -> u64 {
    let min1 = min_side_1(&dice);
    let min2 = min_side_2(&dice);
    let min3 = min_side_3(&dice);

    min3 * 4
    + min2 * (n - 2) * 4
    + min2 * (n - 1) * 4
    + min1 * (n - 2) * (n - 2)
    + min1 * (n - 2) * (n - 1) * 4
}

fn min_side_1(dice: &[u64]) -> u64 {
    dice.iter()
        .copied()
        .min()
        .unwrap()
}

fn min_side_2(dice: &[u64]) -> u64 {
    dice.iter()
        .enumerate()
        .map(|(i, x)| {
            let mut res = u64::MAX;
            for j in (i + 1)..6 {
                if j != 5 - i {
                    res = res.min(x + dice[j]);
                }
            }
            res
        })
        .min()
        .unwrap()
}

fn min_side_3(dice: &[u64]) -> u64 {
    dice.iter()
        .enumerate()
        .map(|(i, x)| {
            let mut res = u64::MAX;
            for j in (i + 1)..6 {
                if j == 5 - i { continue }
                for k in (j + 1)..6 {
                    if k != 5 - i && k != 5 - j {
                        res = res.min(x + dice[j] + dice[k]);
                    }
                }
            }
            res
        })
        .min()
        .unwrap()
}

fn min_side_5(dice: &[u64]) -> u64 {
    let sum: u64 = dice.iter().sum();
    dice.iter()
        .map(|x| sum - x)
        .min()
        .unwrap()
}
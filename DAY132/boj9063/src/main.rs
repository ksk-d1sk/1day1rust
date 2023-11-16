use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut vx = Vec::with_capacity(n);
    let mut vy = Vec::with_capacity(n);

    for (x, y) in (0..n).map(|_| get!(i32, i32)) {
        vx.push(x);
        vy.push(y);
    }

    print!("{}", (max(&vx) - min(&vx)) * (max(&vy) - min(&vy)));
}

fn min<'a, T, I>(rhs: I) -> T
where
    T: Ord + Default + Copy + 'a,
    I: IntoIterator<Item=&'a T>,
{
    rhs.into_iter().copied().min().unwrap_or_default()
}

fn max<'a, T, I>(rhs: I) -> T
where
    T: Ord + Default + Copy + 'a,
    I: IntoIterator<Item=&'a T>,
{
    rhs.into_iter().copied().max().unwrap_or_default()
}
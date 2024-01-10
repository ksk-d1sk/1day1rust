use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = get!(usize, usize);
    let mut v: Vec<_> = (0..n).map(|_| get!(i32)).collect();
    let mut history = Vec::new();

    merge_sort(&mut v, &mut history);

    print!(
        "{}",
        history.get(k - 1).unwrap_or(&-1)
    );
}

fn merge_sort(a: &mut [i32], history: &mut Vec<i32>) {
    if a.len() > 1 {
        let mut temp = Vec::with_capacity(a.len());
        let mid = (a.len() + 1) / 2;

        merge_sort(&mut a[0..mid], history);
        merge_sort(&mut a[mid..], history);

        let left = &a[0..mid];
        let right = &a[mid..];
        let mut i = 0;
        let mut j = 0;

        for _ in 0..a.len() {
            if i == left.len() {
                temp.push(right[j]);
                j += 1;
            } else if j == right.len() {
                temp.push(left[i]);
                i += 1;
            } else if left[i] <= right[j] {
                temp.push(left[i]);
                i += 1;
            } else {
                temp.push(right[j]);
                j += 1;
            }
        }

        for (i, x) in temp.into_iter().enumerate() {
            history.push(x);
            a[i] = x;
        }
    }
}
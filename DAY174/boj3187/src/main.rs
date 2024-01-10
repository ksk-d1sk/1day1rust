use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::repeat;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let v: RefCell<Vec<Vec<_>>> = RefCell::new(
        (0..n).map(|_| get!().bytes().collect()).collect()
    );
    let mut deque = VecDeque::new();
    let mut total_sheep_count = 0;
    let mut total_wolf_count = 0;

    (0..n)
        .flat_map(|x| repeat(x).take(m))
        .zip((0..m).cycle())
        .filter(|&(i, j)| v.borrow()[i][j] == b'v' || v.borrow()[i][j] == b'k')
        .for_each(|(i, j)| {
            let mut sheep_count = 0;
            let mut wolf_count = 0;

            if v.borrow()[i][j] == b'k' {
                sheep_count += 1;
            } else {
                wolf_count += 1;
            }

            (*v.borrow_mut())[i][j] = b'#';
            deque.push_back((i, j));

            while let Some((x, y)) = deque.pop_front() {

                for (nx, ny) in [
                    (x + 1, y), (x - 1, y),
                    (x, y + 1), (x, y - 1),
                ] {
                    if v.borrow()[nx][ny] != b'#' {
                        match v.borrow()[nx][ny] {
                            b'k' => sheep_count += 1,
                            b'v' => wolf_count += 1,
                            _ => {},
                        }

                        (*v.borrow_mut())[nx][ny] = b'#';
                        deque.push_back((nx, ny));
                    }
                }
            }

            if sheep_count > wolf_count {
                total_sheep_count += sheep_count;
            } else {
                total_wolf_count += wolf_count;
            }
        });

    print!("{total_sheep_count} {total_wolf_count}");
}
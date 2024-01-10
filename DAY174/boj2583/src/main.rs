use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Write;
use std::io;
use std::iter::repeat;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m, k) = get!(usize, usize, usize);
    let visit = RefCell::new(vec![vec![true; m]; n]);
    let mut deque = VecDeque::new();
    let mut v_area = Vec::new();
    let mut count = 0_usize;

    for (x1, y1, x2, y2) in (0..k).map(|_| get!(usize, usize, usize, usize)) {
        for i in y1..y2 {
            for j in x1..x2 {
                (*visit.borrow_mut())[i][j] = false;
            }
        }
    }

    (0..n)
        .flat_map(|x| repeat(x).take(m))
        .zip((0..m).cycle())
        .filter(|&(i, j)| visit.borrow()[i][j])
        .for_each(|(i, j)| {
            let mut area = 0;

            count += 1;
            deque.push_back((i, j));
            (*visit.borrow_mut())[i][j] = false;

            while let Some((x, y)) = deque.pop_front() {
                area += 1;

                for (option_nx, option_ny) in [
                    (x.checked_add(1), y.checked_add(0)),
                    (x.checked_sub(1), y.checked_add(0)),
                    (x.checked_add(0), y.checked_add(1)),
                    (x.checked_add(0), y.checked_sub(1)),
                ] {
                    if let (Some(nx), Some(ny)) = (option_nx, option_ny) {
                        if nx < n && ny < m && visit.borrow()[nx][ny] {
                            (*visit.borrow_mut())[nx][ny] = false;
                            deque.push_back((nx, ny));
                        }
                    }
                }
            }

            v_area.push(area);
        });

    v_area.sort_unstable();
    for area in v_area {
        let _ = write!(output, "{area} ");
    }

    print!("{count}\n{output}");
}
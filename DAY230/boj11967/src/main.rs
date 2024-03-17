use std::collections::{HashMap, VecDeque};
use std::io::*;

#[derive(Clone, Copy, PartialEq)]
enum Object {
    Bright,
    Darkness,
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let mut grid = vec![vec![Object::Darkness; n]; n];
    let mut map: HashMap<_, Vec<_>> = HashMap::with_capacity(m);
    let mut queue = VecDeque::new();
    let mut answer = 1;
    let mut light_on_check = true;

    for (x, y, a, b) in (0..m).map(|_| next!(usize, usize, usize, usize)) {
        let (x, y, a, b) = (x - 1, y - 1, a - 1, b - 1);
        map.entry((x, y)).and_modify(|v| v.push((a, b))).or_insert(vec![(a, b)]);
    }

    grid[0][0] = Object::Bright;

    while light_on_check {
        let mut visited = vec![vec![false; n]; n];

        light_on_check = false;
        visited[0][0] = true;
        queue.push_back((0, 0));

        while let Some((x, y)) = queue.pop_front() {
            while let Some((a, b)) = map.get_mut(&(x, y)).and_then(|v| v.pop()) {
                if grid[a][b] != Object::Bright {
                    answer += 1;
                    light_on_check = true;
                    grid[a][b] = Object::Bright;
                }
            }

            for (nx, ny) in [
                (x + 1, y), (x, y + 1),
                (x - 1, y), (x, y - 1),
            ] {
                if nx < n && ny < n && !visited[nx][ny] && grid[nx][ny] == Object::Bright {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    print!("{answer}");
}
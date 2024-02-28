use std::collections::VecDeque;
use std::io::*;

use Kuni::*;

enum Kuni {
    Island(u16), // 섬과 섬의 번호
    Bridge(u16), // i번 섬의 다리 (0은 바다)
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut kuni: Vec<Vec<_>> = (0..n)
        .map(|_| (0..n)
            .map(|_| if next!(u8) == 1 { Island(0) } else { Bridge(0) })
            .collect()
        )
        .collect();
    let mut queue1 = VecDeque::new();
    let mut queue2 = VecDeque::new();
    let mut answer = u16::MAX;
    let mut number = 1;

    for i in 0..n {
        for j in 0..n {
            if let Island(0) = kuni[i][j] {
                kuni[i][j] = Island(number);
                queue1.push_back((i, j));

                // 섬에 대한 BFS
                while let Some((x, y)) = queue1.pop_front() {
                    for (nx, ny) in [
                        (x + 1, y), (x, y + 1),
                        (x - 1, y), (x, y - 1),
                    ] {
                        if nx < n && ny < n {
                            match &mut kuni[nx][ny] {
                                Island(num) if *num == 0 => {
                                    *num = number;
                                    queue1.push_back((nx, ny));
                                },
                                Bridge(num) if *num < number => {
                                    *num = number;
                                    queue2.push_back((nx, ny, 1));
                                },
                                _ => {},
                            }
                        }
                    }
                }

                // 다리를 잇는 BFS
                while let Some((x, y, cnt)) = queue2.pop_front() {
                    for (nx, ny) in [
                        (x + 1, y), (x, y + 1),
                        (x - 1, y), (x, y - 1),
                    ] {
                        if nx < n && ny < n {
                            match &mut kuni[nx][ny] {
                                Island(num) if *num != number => {
                                    answer = answer.min(cnt);
                                    queue2.clear();
                                    break;
                                },
                                Bridge(num) if *num < number => {
                                    *num = number;
                                    queue2.push_back((nx, ny, cnt + 1));
                                },
                                _ => {},
                            }
                        }
                    }
                }

                number += 1;
            }
        }
    }

    print!("{answer}");
}
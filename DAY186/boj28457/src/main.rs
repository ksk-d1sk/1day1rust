use std::collections::VecDeque;
use std::io;

use UtilMarble::*;

#[derive(Debug)]
enum UtilMarble {
    City(i32),
    Sold,
    GoldenCard,
    Start,
    UIsland,
    WelfareFund,
    Travel,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, mut s, w, g) = get!(usize, i32, i32, usize);
    let golden_card_list: Vec<_> = (0..g).map(|_| get!(u8, i32)).collect();
    let mut golden_card_cycle = golden_card_list.into_iter().cycle();
    let size = n * 4 - 4;
    let mut board = Vec::with_capacity(size);
    let mut city_count = 0;

    let mut special = [Start, UIsland, WelfareFund, Travel].into_iter();

    // 보드게임 판 세팅
    for i in 0..size {
        if i % (n - 1) == 0 {
            board.push(special.next().unwrap());
        } else {
            let normal = get!();
            if normal == "L" {
                let price = get!(i32);
                city_count += 1;
                board.push(City(price));
            } else {
                board.push(GoldenCard);
            }
        }
    }

    let i = get!(u8);
    let mut deque = VecDeque::from_iter((0..i).map(|_| get!(usize, usize)));
    let mut cur = 0;                  // 현재 위치 인덱스
    let mut rest_turn = 0;            // 쉬는 턴
    let mut welfare_fund_funds = 0;   // 사회복지기금에 쌓은 돈
    let mut check = true;

    // 보드게임 시작
    while let Some((a, b)) = deque.pop_front() {
        if rest_turn == 0 {
            cur += a + b;
            s +=  w * (cur / size) as i32;
            cur %= size;

            match board[cur] {
                Sold => {/*  this city is sold  */},
                Start => {/*  Happy payday ;)  */},
                UIsland => { rest_turn += 3 },
                WelfareFund => {
                    s += welfare_fund_funds;
                    welfare_fund_funds = 0;
                },
                Travel => {
                    s += w;
                    cur = 0;
                },
                City(price) => {
                    if s >= price {
                        s -= price;
                        board[cur] = Sold;
                        city_count -= 1;
                    }
                },
                GoldenCard => {
                    let (kind, num) = golden_card_cycle.next().unwrap();

                    match kind {
                        1 => { s += num },
                        2 => { s -= num },
                        3 => {
                            welfare_fund_funds += num;
                            s -= num;
                        },
                        4 => {
                            deque.push_front((0, num as usize));
                        },
                        _ => unreachable!(),
                    }

                    if s < 0 {
                        check = false;
                        break;
                    }
                },
            }
        } else if a == b {
            rest_turn = 0;
        } else {
            rest_turn -= 1;
        }
    }

    if check && city_count == 0 {
        print!("WIN");
    } else {
        print!("LOSE")
    }
}
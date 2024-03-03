use std::collections::VecDeque;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (m, n, o, p, q, r, s, t, u, v, w) = next!(usize, usize, usize, usize, usize, usize,
                                                  usize, usize, usize, usize, usize);
    let mut tomato = vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![0; m]; n]; o]; p]; q]; r]; s]; t]; u]; v]; w];
    let mut queue = VecDeque::new();
    let mut answer = 0_u32;

    for wi in 0..w {
        for vi in 0..v {
            for ui in 0..u {
                for ti in 0..t {
                    for si in 0..s {
                        for ri in 0..r {
                            for qi in 0..q {
                                for pi in 0..p {
                                    for oi in 0..o {
                                        for ni in 0..n {
                                            for mi in 0..m {
                                                let temp = next!(i8);
                                                if temp == 1 {
                                                    queue.push_back((wi, vi, ui, ti, si, ri, qi, pi, oi, ni, mi, 1));
                                                }
                                                tomato[wi][vi][ui][ti][si][ri][qi][pi][oi][ni][mi] = temp;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    while let Some((wk, vk, uk, tk, sk, rk, qk, pk, ok ,nk, mk, cnt)) = queue.pop_front() {
        for (nw, nv, nu, nt, ns, nr, nq, np, no, nn, nm) in [
            (wk - 1, vk, uk, tk, sk, rk, qk, pk, ok ,nk, mk), (wk + 1, vk, uk, tk, sk, rk, qk, pk, ok ,nk, mk),
            (wk, vk - 1, uk, tk, sk, rk, qk, pk, ok ,nk, mk), (wk, vk + 1, uk, tk, sk, rk, qk, pk, ok ,nk, mk),
            (wk, vk, uk - 1, tk, sk, rk, qk, pk, ok ,nk, mk), (wk, vk, uk + 1, tk, sk, rk, qk, pk, ok ,nk, mk),
            (wk, vk, uk, tk - 1, sk, rk, qk, pk, ok ,nk, mk), (wk, vk, uk, tk + 1, sk, rk, qk, pk, ok ,nk, mk),
            (wk, vk, uk, tk, sk - 1, rk, qk, pk, ok ,nk, mk), (wk, vk, uk, tk, sk + 1, rk, qk, pk, ok ,nk, mk),
            (wk, vk, uk, tk, sk, rk - 1, qk, pk, ok ,nk, mk), (wk, vk, uk, tk, sk, rk + 1, qk, pk, ok ,nk, mk),
            (wk, vk, uk, tk, sk, rk, qk - 1, pk, ok ,nk, mk), (wk, vk, uk, tk, sk, rk, qk + 1, pk, ok ,nk, mk),
            (wk, vk, uk, tk, sk, rk, qk, pk - 1, ok ,nk, mk), (wk, vk, uk, tk, sk, rk, qk, pk + 1, ok ,nk, mk),
            (wk, vk, uk, tk, sk, rk, qk, pk, ok - 1 ,nk, mk), (wk, vk, uk, tk, sk, rk, qk, pk, ok + 1 ,nk, mk),
            (wk, vk, uk, tk, sk, rk, qk, pk, ok ,nk - 1, mk), (wk, vk, uk, tk, sk, rk, qk, pk, ok ,nk + 1, mk),
            (wk, vk, uk, tk, sk, rk, qk, pk, ok ,nk, mk - 1), (wk, vk, uk, tk, sk, rk, qk, pk, ok ,nk, mk + 1),
        ] {
            if nn < n && nm < m && no < o && np < p && nq < q && nr < r && ns < s && nt < t && nu < u && nv < v && nw < w {
                if tomato[nw][nv][nu][nt][ns][nr][nq][np][no][nn][nm] == 0 {
                    tomato[nw][nv][nu][nt][ns][nr][nq][np][no][nn][nm] = 1;
                    answer = cnt;
                    queue.push_back((nw, nv, nu, nt, ns, nr, nq, np, no, nn, nm, cnt + 1));
                }
            }
        }
    }

    let check = tomato.iter()
        .flatten().flatten().flatten().flatten().flatten()
        .flatten().flatten().flatten().flatten().flatten()
        .all(|x| *x != 0);

    if check {
        print!("{answer}");
    } else {
        print!("-1");
    }
}
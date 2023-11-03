fn main() {
    // 두 Iterator를 합치는 메서드
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];
    let chain: Vec<i32> = a1.iter().chain(a2.iter()).map(|e| *e).collect();
    assert_eq!(chain, [1, 2, 3, 4, 5, 6]);

    // Iterator가 아니더라도 이터러블한 타입이라면 인자로 사용할 수 있다
    let v: Vec<i32> = (1..4).chain([20, 30, 40]).collect();
    assert_eq!(v, [1, 2, 3, 20, 30, 40]);

    let v: Vec<i32> = (1..4).chain([20, 30, 40]).rev().collect();
    assert_eq!(v, [40, 30, 20, 3, 2, 1]);
}

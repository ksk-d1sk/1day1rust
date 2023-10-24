fn main() {
    let text = "  ponies  \n  giraffes\niguanas  \nsquid";
    let v: Vec<&str> = text.lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);

    // 위의 이터레이터 코드는 다음 코드만큼 효율적으로 인라인 처리된다
    let mut v = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line != "iguanas" {
            v.push(line);
        }
    }
    assert_eq!(v, ["ponies", "giraffes", "squid"]);

    // next를 호출하기 전까지 map의 클로저가 호출되지 않는다
    ["earth", "water", "air", "fire"]
        .iter().map(|elt| println!("{}", elt));
}

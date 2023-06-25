fn main() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    assert_eq!(v,  ["101", "104", "substitute"]);

    let v = vec![
        "kimchi".to_string(),
        "BTS".to_string(),
        "bulgogi".to_string(),
        "PSY".to_string()
    ];

    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    let mut composers = Vec::new();
    composers.push(Person { name: Some("Palestring".to_string()), birth: 1525});

    // let first_name_name = composers[0].name;

    // let first_name = std::mem::replace(&mut composers[0].name, None);
    // assert_eq!(first_name, Some("Palestring".to_string()));
    // assert_eq!(composers[0].name, None);

    let first_name = composers[0].name.take();
    println!("{:?}", first_name);
}

struct Person {
    name: Option<String>,
    birth: i32,
}
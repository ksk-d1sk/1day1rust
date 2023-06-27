use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();

    table.insert(
        "Gesualdo".to_string(),
        vec!["many madrigals".to_string(), "Tenbrae Responsoria".to_string()]
    );
    table.insert(
        "Caravaggio".to_string(),
        vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]
    );
    table.insert(
        "Cellini".to_string(),
        vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]
    );

    sort_works(&mut table);
    show(&table);
//////////////////////////////////////////
    let x = 10;
    let r = &x;
    assert!(*r == 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m == 64);
//////////////////////////////////////////
    let aria = Anime{
        name: "Aria: The Animation",
        bechdel_pass: true
    };

    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!((*anime_ref).name, "Aria: The Animation");
//////////////////////////////////////////
    let mut v = vec![1973, 1968];
    v.sort_unstable();
    (&mut v).sort_unstable();
//////////////////////////////////////////
    let x = 10;
    let y = 20;
    let mut r = &x;

    if true { r = &y }

    assert!(*r == 10 || *r == 20);
//////////////////////////////////////////
    let point = Point {
        x: 1000,
        y: 729,
    };
    let r = &point;
    let rr = &r;
    let rrr = &rr;

    assert_eq!(rrr.y,  729);
//////////////////////////////////////////
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(rx == ry);
    assert!(!std::ptr::eq(rx, ry));

    // assert!(rx == rrx);    // error!
    assert!(rx == *rrx);
}

struct Point {
    x: i32,
    y: i32,
}

struct Anime {
    name: &'static str,
    bechdel_pass: bool,
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" : {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_, works) in table {
        works.sort_unstable();
    }
}

use std::ops::IndexMut;

fn main() {
    let mut desserts = vec!["Howalon".to_string(), "Soan papdi".to_string()];

    // mut 참조 발생
    desserts[0].push_str(" (fictional) ");
    desserts[1].push_str(" (real) ");

    //아래 코드와 동일하다
    (*desserts.index_mut(0)).push_str(" (fictional) ");
    (*desserts.index_mut(1)).push_str(" (real) ");
}

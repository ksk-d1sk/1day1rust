fn main() {
    let bee_parts = ["head", "thorax", "abdomen"];

    let mut iter = bee_parts.into_iter();
    assert_eq!(iter.next()     , Some("head")   );
    assert_eq!(iter.next_back(), Some("abdomen"));
    assert_eq!(iter.next()     , Some("thorax") );

    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next()     , None);

    let meals = ["breakfast", "lunch", "dinner"];

    let mut iter = meals.into_iter().rev();
    assert_eq!(iter.next(), Some("dinner"));
    assert_eq!(iter.next(), Some("lunch"));
    assert_eq!(iter.next(), Some("breakfast"));
    assert_eq!(iter.next(), None);
}

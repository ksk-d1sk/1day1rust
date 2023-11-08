fn main() {
    let bytes = "Xerxes";
    let mut iter = bytes.chars();
    assert_eq!(iter.position(|c| c == 'r'), Some(2));
    assert_eq!(iter.next(), Some('x'));
    assert_eq!(iter.next(), Some('e'));
    assert_eq!(iter.next(), Some('s'));

    assert_eq!(bytes.chars().position(|c| c == 'z'), None);
}

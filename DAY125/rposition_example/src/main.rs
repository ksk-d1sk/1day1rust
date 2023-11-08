fn main() {
    let bytes = b"Xerxes";
    let mut iter = bytes.iter().copied();
    assert_eq!(iter.rposition(|c| c == b'r'), Some(2));
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next(), Some(b'X'));
    assert_eq!(iter.next(), Some(b'e'));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let mut iter = bytes.iter().copied();
    assert_eq!(iter.rposition(|c| c == b'X'), Some(0));
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next(), None);
}

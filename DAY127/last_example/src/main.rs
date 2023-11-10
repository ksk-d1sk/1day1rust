fn main() {
    let squares = (0..10).map(|i| i * i);
    assert_eq!(squares.last(), Some(81));
}

fn main() {
    let packed  = "Helen of Troy";
    let spaced  = "Helen   of    Troy";
    let obscure = "Helen of Sandusky";

    assert!(packed != spaced);
    assert!(packed.split_whitespace().eq(spaced.split_whitespace()));

    assert!(spaced < obscure);
    assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));
}

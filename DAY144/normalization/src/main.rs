use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn hash<T>(t: &T) -> u64
where
    T: ?Sized + Hash,
{
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    println!("th\u{e9} : the\u{301}");
    assert!("th\u{e9}" != "the\u{301}");
    assert!("th\u{e9}" > "the\u{301}");

    let hash1 = hash("th\u{e9}");
    let hash2 = hash("the\u{301}");
    println!("{:#x}", hash1);
    println!("{:#x}", hash2);
    assert_ne!(hash1, hash2);
}

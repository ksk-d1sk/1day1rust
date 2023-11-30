use unicode_normalization::UnicodeNormalization;

fn main() {
    println!("{}", "① Di\u{fb03}culty".nfkc().collect::<String>());
}

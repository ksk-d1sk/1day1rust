use regex::Regex;

fn main() {
    let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").unwrap();
    let haystack = r#"regex = "0.2.5"#;
    assert!(semver.is_match(haystack));

    let captures = semver.captures(haystack)
        .ok_or("semver regex should have matched")
        .unwrap();

    assert_eq!(&captures[0], "0.2.5");
    assert_eq!(&captures[1], "0");
    assert_eq!(&captures[2], "2");
    assert_eq!(&captures[3], "5");

    assert_eq!(captures.get(4), None);
    assert_eq!(captures.get(3).unwrap().start() , 13);
    assert_eq!(captures.get(3).unwrap().end()   , 14);
    assert_eq!(captures.get(3).unwrap().as_str(), "5");

    let haystack = "In the beginning, there was 1.0.0. \
                    For a while, we used 1.0.1-beta, \
                    but in the end, we settled on 1.2.4.";
    let matches: Vec<&str> = semver.find_iter(haystack)
        .map(|match_| match_.as_str())
        .collect();

    println!("{}", haystack);
    assert_eq!(matches, ["1.0.0", "1.0.1-beta", "1.2.4"]);
}
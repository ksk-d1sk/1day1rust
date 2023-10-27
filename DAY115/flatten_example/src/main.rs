use std::collections::BTreeMap;

fn main() {
    let mut parks = BTreeMap::new();

    parks.insert("Portland", vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);

    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();

    assert_eq!(
        all_parks,
        ["Tadasu-no-Mori Forest", "Maruyama Koen", "Percy Warner Park", "Dragon Park", "Mt. Tabor Park", "Forest Park"]
    );

    assert_eq!(
        vec![None, Some("day"), None, Some("one")]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
        vec!["day", "one"]
    );
}
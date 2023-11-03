fn main() {
    let upper_case: String = "große".chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!(" after:     {:?}", c))
        .collect();
    assert_eq!(upper_case, "GROSSE");
//         before: 'g'
//          after:     'G'
//         before: 'r'
//          after:     'R'
//         before: 'o'
//          after:     'O'
//         before: 'ß'
//          after:     'S'
//          after:     'S'
//         before: 'e'
//          after:     'E'

    let a = [1, 4, 2, 3];
    let sum = a.iter()
        .cloned()
        .inspect(|x| println!("about to filter: {x}"))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("made it through filter: {x}"))
        .fold(0, |sum, i| sum + i);
    assert_eq!(sum, 6);

    let lines = ["1", "2", "a"];
    let sum: i32 = lines
        .iter()
        .map(|line| line.parse::<i32>())
        .inspect(|num| {
            if let Err(ref e) = *num {
                println!("Parsing error: {e}");
            }
        })
        .filter_map(Result::ok)
        .sum();
    assert_eq!(sum, 3);
}

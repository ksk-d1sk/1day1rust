fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let half_life: u16 = buf.trim().parse().unwrap();

    print!(
        "{}",
        match half_life {
            620.. => "Red",
            590.. => "Orange",
            570.. => "Yellow",
            495.. => "Green",
            450.. => "Blue",
            425.. => "Indigo",
            0..   => "Violet",
        }
    )
}

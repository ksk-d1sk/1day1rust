fn main() {
    let speech = "\"Ouch!\" said the well.\n";
    println!("{}",  speech);

    println!("In the room the women come and go,
        Singing of Mount Abora\n");

    println!("It was a bright, cold day in April, and \
        there were four of us-\
        more or less.\n");

    let default_win_install_path = r"C:\Program Files\Gorillas";
    println!("{default_win_install_path}\n");

    println!(r###"
        This raw string started with 'r###"'.
        Therefore it does not end until we reach a quote mark ('"')
        followed immediately by three pound signs ('###'):
    "###);

    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    println!("{}\n", oodles);
}

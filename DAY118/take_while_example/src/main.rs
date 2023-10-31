fn main() {
    let message = "To: jimb\r\n\
                   From: superego <editor@oreilly.com>\r\n\
                   \r\n\
                   Did you get any writing done today?\r\n\
                   When will you stop wasting time plotting fractals?\r\n";

    for header in message.lines().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }
}
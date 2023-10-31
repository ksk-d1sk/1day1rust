fn main() {
    let message = "To: jimb\r\n\
                   From: superego <editor@oreilly.com>\r\n\
                   \r\n\
                   Did you get any writing done today?\r\n\
                   When will you stop wasting time plotting fractals?\r\n";

    for body in message.lines()
        .skip_while(|l| !l.is_empty())
        .skip(1) {
        println!("{}", body);
    }
}
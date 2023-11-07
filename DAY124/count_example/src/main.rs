fn main() {
    let message = "To: jimb\r\n\
                   From: superego <editor@oreilly.com>\r\n\
                   Did you get any writing done today?\r\n\
                   When will you stop wasting time plotting fractals?\r\n";

    assert_eq!(message.lines().count(), 4);
}

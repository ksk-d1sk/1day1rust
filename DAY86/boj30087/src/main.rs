use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for seminar in buf.split_ascii_whitespace().skip(1) {
        output.push_str(
            match seminar {
                "Algorithm" => "204",
                "DataAnalysis" => "207",
                "ArtificialIntelligence" => "302",
                "CyberSecurity" => "B101",
                "Network" => "303",
                "Startup" => "501",
                "TestStrategy" => "105",
                _ => unreachable!(),
            }
        );
        output.push('\n');
    }

    print!("{output}");
}

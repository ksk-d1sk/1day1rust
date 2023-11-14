use std::error::Error;
use std::fmt::Write;

fn main() -> Result<(), Box<dyn Error>> {
    ["doves", "hens", "birds"].iter()
        .zip(["turtle", "french", "calling"])
        .zip(2..5)
        .rev()
        .map(|((item, kind), quantity)| {
            format!("{} {} {}", quantity, kind, item)
        })
        .for_each(|gift| {
            println!("You have received: {}", gift);
        });

    println!();
    let mut output = String::new();

    ["doves", "hens", "birds"].iter()
        .zip(["turtle", "french", "calling"])
        .zip(2..5)
        .rev()
        .map(|((item, kind), quantity)| {
            format!("{} {} {}", quantity, kind, item)
        })
        .try_for_each(|gift| {
            writeln!(output, "You have receivec: {}", gift)
        })?;

    println!("{output}");

    Ok(())
}

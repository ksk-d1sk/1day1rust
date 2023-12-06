use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, Write, BufWriter};

use rand::Rng;

fn main() -> io::Result<()> {
    writeln!(io::stderr(), "error: world not helloalbe")?;

    let mut byte_vec = vec![];
    write!(&mut byte_vec, "{0}~ {0}~ {0}~ ♪♪♪♪ ♪ ♪", "해피")?;
    println!("{:?}", byte_vec);
    if let Some(str) = String::from_utf8(byte_vec).ok() {
        println!("{}", str);
    }

    let file = File::create("lotto.txt")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "로또 추첨기 ^^")?;
    let mut list: VecDeque<u32> = (1..=45).collect();

    for i in 0..6 {
        let idx = rand::thread_rng().gen_range(0..(45 - i));
        write!(writer, "{} ", list.remove(idx).unwrap())?;
    }

    Ok(())
}

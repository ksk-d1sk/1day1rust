use std::io;
use std::io::Write;
use std::hash::Hash;
use std::fmt::Debug;

// fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {  // usually function
//     out.write_all(b"hello world\n")?;
//     out.flush()
// }

fn say_hello<W: Write>(out: &mut W) -> io::Result<()> {        // genetic function
    out.write_all(b"hello world\n")?;
    out.flush()
}

// fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {  // not using the where keyword
//     // do something
// }

fn top_ten<T>(values: &Vec<T>)                          // using the where keyword
where
    T: Debug + Hash + Eq
{  //
    // do something
}

trait MeasureDistance {}

// 수명 제네릭과 타입 제네릭을 함께 선언할 때는 수명 제네릭을 먼저 선언한다.
fn nearest<'t, 'c, P>(target: &'t P, candidates: &'c [P]) -> &'c P
where
    P: MeasureDistance
{
    panic!("do something")
}

fn main() -> io::Result<()> {
    let mut local_file = std::fs::File::create("hello.txt")?;
    let mut bytes = Vec::new();

    say_hello(&mut local_file)?;
    say_hello(&mut bytes)?;

//  let v1 = (0..1000).collect();              // error: 타입을 추론할 수 없다.
    let v2 = (0..1000).collect::<Vec<i32>>();  // Ok!

    Ok(())
}

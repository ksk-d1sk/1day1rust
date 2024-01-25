use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

// ???

fn main() {
    let buffer = Arc::new(Mutex::new(String::new()));
    let filenames = vec![
        String::from("A"), "B".to_string(),
        String::from("C"), "D".to_string(),
        String::from("E"), "F".to_string(),
        String::from("G"), "H".to_string(),
        String::from("I"), "J".to_string(),
        String::from("K"), "L".to_string(),
        String::from("M"), "N".to_string(),
        String::from("O"), "P".to_string(),
        String::from("Q"), "R".to_string(),
        String::from("S"), "T".to_string(),
        String::from("U"), "V".to_string(),
        String::from("W"), "X".to_string(),
        String::from("Y"), "Z".to_string(),
    ];

    let _ = process_files_in_parallel(filenames, buffer.clone());
    println!();
    println!("{}", buffer.lock().unwrap());
}

fn process_files_in_parallel(filenames: Vec<String>, buf: Arc<Mutex<String>>) -> io::Result<()> {
    const NTHREADS: usize = 8;
    let worklists = filenames.chunks(NTHREADS);

    let mut thread_handles = Vec::new();
    for worklist in worklists {
        let worklist = worklist.to_vec();
        let buf_child = buf.clone();
        thread_handles.push(
            thread::spawn(move || process_files(worklist, buf_child))
        );
    }

    for handle in thread_handles {
        handle.join().unwrap()?
    }

    Ok(())
}

fn process_files(filenames: Vec<String>, buf: Arc<Mutex<String>>) -> io::Result<()> {
    for document in filenames {
        println!("{}파일에 무언가 작업중..", document);
        let mut data = buf.lock().unwrap();
        data.push_str(&document);
    }

    Ok(())
}
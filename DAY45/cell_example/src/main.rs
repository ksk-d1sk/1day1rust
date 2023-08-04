use std::cell::RefCell;
use std::io::Write;
use std::fs::File;

pub struct SpiderRobot {
    log_file: RefCell<File>,
}

impl SpiderRobot {
    pub fn log(&self, mesasge: &str) {
        let mut file = self.log_file.borrow_mut();
        writeln!(file, "{}", mesasge).unwrap();
    }
}

fn main() {}

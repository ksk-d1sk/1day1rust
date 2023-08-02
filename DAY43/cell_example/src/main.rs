use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::Write;

pub struct SpiderRobot {
    hardware_error_count: Cell<u32>,
    log_file: RefCell<File>,
}

impl SpiderRobot {
    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }

    pub fn has_hardware_errors(&self) -> bool {
        self.hardware_error_count.get() > 0
    }

    pub fn log(&self, message: &str) {
        let mut file = self.log_file.borrow_mut();
        writeln!(file, "{}", message).unwrap();
    }
}

fn main() {
    let ref_cell: RefCell<String> = RefCell::new("Hello".to_string());

    let r = ref_cell.borrow();
    let count = r.len();
    assert_eq!(count, 5);

    let mut w = ref_cell.borrow_mut();     // 코드 실행 시 여기서 패닉 발생!
    w.push_str(" world");
}

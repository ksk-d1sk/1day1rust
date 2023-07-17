pub const PI: f64 = 3.1415926535897;
pub static mut PACKETS_SERVED: usize = 0;

fn main() {
    // println!("{} served",  PACKETS_SERVED);  // error! mut static 변수를 사용
}

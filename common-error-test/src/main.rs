use crate::_common_error::{SYS00001, SYS00002};

mod _common_error;
fn main() {
    let throw1 = SYS00001::throw("cc");
    println!("Hello, world!");
}

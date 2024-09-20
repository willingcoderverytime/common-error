use crate::_common_error::SYS00001;

mod _common_error;
fn main() {
    let _throw1 = SYS00001::throw("cc");
    println!("Hello, world!");
}

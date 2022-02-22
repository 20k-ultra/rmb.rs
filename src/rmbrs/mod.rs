pub mod link;
pub mod timer;
pub mod todo;

mod data;

pub fn print() {
    let d = data::read();
    println!("Data: {d}");
}

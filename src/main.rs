use std::{thread, time};

fn main() {
    println!("This is a simple demo.");

    let time2wait = time::Duration::from_millis(5000);
    thread::sleep(time2wait);
}

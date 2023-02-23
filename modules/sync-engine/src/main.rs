use std::{thread, time};

fn main() {
    loop {
        println!("Ergonames sync engine");
        thread::sleep(time::Duration::from_secs(2));
    }
}

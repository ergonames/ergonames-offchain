use std::{thread, time};
use anyhow::Result;

fn main() {
    loop {
        let box_id = "kfdkfjdkfkdf";
        let output = run_jar(box_id);
        println!("{:?}", output);
        thread::sleep(time::Duration::from_secs(2));
    }
}

fn run_jar(box_id: &str) -> Result<String> {
    let opt = std::process::Command::new("java")
        .args(&["-jar", "ergonames-transaction-utils.jar", box_id])
        .output()
        .expect("Failed to run ergonames-transaction-utils.jar");

    let output = String::from_utf8_lossy(&opt.stdout);
    if output.contains("Error") {
        return Err(anyhow::anyhow!("Error"));
    }
    Ok(output.to_string())
}
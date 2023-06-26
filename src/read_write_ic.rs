use std::{process::Command, io};

pub fn write_data(file_path: &str, oid: &str) -> io::Result<()> {
    let write_output = Command::new("./bin/trustm_data")
        .arg("-w")
        .arg(oid)
        .arg("-i")
        .arg(file_path)
        .output()?;

    if write_output.status.success() {
        println!("Write Success.");
    } else {
        let stderr = String::from_utf8_lossy(&write_output.stderr);
        println!("Command executed with error: {}", stderr);
        return Ok(());
    }

    Ok(())
}

pub fn read_data(oid: &str) -> io::Result<()> {
    let read_output = Command::new("./bin/trustm_data")
        .arg("-r")
        .arg(oid)
        .output()?;

    if read_output.status.success() {
        let stdout = String::from_utf8_lossy(&read_output.stdout);
        println!("Read Output:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&read_output.stderr);
        println!("Command executed with error: {}", stderr);
    }

    Ok(())
}

// fn main() -> io::Result<()> {
//     let file_path = "./1234.txt";
//     let oid = "0xe0e1";

//     write_data(file_path, oid)?;
//     read_data(oid)?;

//     Ok(())
// }
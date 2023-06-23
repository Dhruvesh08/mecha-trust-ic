use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Open the file with an absolute path
    let mut file = File::open("/MECHA_TEST/optiga_trust_m/trustm_chipinfo")?;

    // Read the contents of the file into a buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Process the data in the buffer
    // Convert the bytes to a string
    let output = String::from_utf8_lossy(&buffer);

    // Print the captured output
    println!("{}", output);

    Ok(())
}

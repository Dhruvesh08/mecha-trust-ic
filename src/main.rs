use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Open the file with an absolute path
    let mut file = File::open("/MECHA_TEST/optiga_trust_m/trustm_chipinfo")?;

    // Read the contents of the file into a buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Process the data in the buffer
    // For example, you can print the bytes
    for byte in buffer {
        println!("{}", byte);
    }

    Ok(())
}

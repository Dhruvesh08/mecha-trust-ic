use std::process::Command;
use std::io::{self, Write};

// Key type enum
#[derive(Debug)]
enum KeyType {
    Auth,
    Enc,
    Hfwu,
    DevM,
    Sign,
    Agmt,
}

// Key size enum
#[derive(Debug)]
enum KeySize {
    ECC256,
    ECC384,
    ECC521,
    Brainpool256,
    Brainpool384,
    Brainpool512,
}

// Generate ECC key pair using trustm_ecc_keygen binary
fn generate_key_pair(oid: &str, key_type: KeyType, key_size: KeySize,file_name: &str) -> Result<String, io::Error> {
    let type_code = match key_type {
        KeyType::Auth => "0x01",
        KeyType::Enc => "0x02",
        KeyType::Hfwu => "0x04",
        KeyType::DevM => "0x08",
        KeyType::Sign => "0x10",
        KeyType::Agmt => "0x20",
    };

    let size_code = match key_size {
        KeySize::ECC256 => "0x03",
        KeySize::ECC384 => "0x04",
        KeySize::ECC521 => "0x05",
        KeySize::Brainpool256 => "0x13",
        KeySize::Brainpool384 => "0x15",
        KeySize::Brainpool512 => "0x16",
    };

    let output = Command::new("/MECHA_TEST/optiga_trust_m/trustm_ecc_keygen")
        .args(&["-g", oid, "-t", type_code, "-k", size_code, "-o", file_name ,"-s"])
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(io::Error::new(io::ErrorKind::Other, stderr))
    }
}

// Extract public key from the output string
fn extract_public_key(output: &str) -> Option<String> {
    let lines: Vec<&str> = output.trim().lines().collect();
    for line in lines.iter().rev() {
        if line.starts_with("-----BEGIN PUBLIC KEY-----") {
            return Some(line.to_string());
        }
    }
    None
}

// Display the public key
fn display_public_key(public_key: &str) {
    println!("Public Key:\n{}", public_key);
}


// fn main() {
//     let oid = "0xe0f3"; // Replace with the desired OID
//     let key_type = KeyType::Auth; // Replace with the desired key type
//     let key_size = KeySize::ECC256; // Replace with the desired key size
//     let file_name:&str = "pub_key.pem";
//     // Generate key pair
//     let output = match generate_key_pair(oid, key_type, key_size,file_name) {
//         Ok(output) => output,
//         Err(err) => {
//             eprintln!("Error generating key pair: {}", err);
//             return;
//         }
//     };


//     //println!("output: {}", output);
//     println!("output: {}", output);

//     // Extract public key
//     let public_key = match extract_public_key(&output) {
//         Some(public_key) => public_key,
//         None => {
//             eprintln!("Error extracting public key");
//             return;
//         }
//     };

//     // Display the public key
//     display_public_key(&public_key);
// }


use std::process::Command;

fn verify_signature_using_public_key(pubkey_file: &str, data_file: &str, sig_file: &str) -> bool {
   
    let output = std::process::Command::new("trustm_ecc_verify")
        .arg("-p")
        .arg(pubkey_file)
        .arg("-i")
        .arg(data_file)
        .arg("-s")
        .arg(sig_file)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).contains("Verify Success.")
}

fn verify_signature_using_certificate(cert_oid: &str, data_file: &str, sig_file: &str) -> bool {
    let output = Command::new("trustm_ecc_verify")
        .arg("-k")
        .arg(cert_oid)
        .arg("-i")
        .arg(data_file)
        .arg("-s")
        .arg(sig_file)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).contains("Verify Success.")
}

// fn main() {
//     // Example usage
//     let is_verified_using_public_key = verify_signature_using_public_key(
//             "test_e0f3_pub.pem",
//             "data.txt",
//             "sig.bin"
//         );
    
//      println!(
//         "Verify signature using public key only: {}",
//         is_verified_using_public_key
//     );
    
//      let is_verified_using_certificate = verify_signature_using_certificate(
//             "0xe0e3",
//             "data.txt",
//             "sig.bin"
//         );
    
//      println!(
//         "Verify signature using certificate: {}",
//         is_verified_using_certificate
//     );
// }

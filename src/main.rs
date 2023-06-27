use std::process::Command;
use std::io::{self, Write};

fn sign_file_with_key(
    key_oid: &str,
    output_file: &str,
    input_file: &str,
    hash: bool,
) -> Result<(Vec<u8>), String> {
    let mut command = Command::new("/MECHA_TEST/optiga_trust_m/trustm_ecc_sign");
    command.args(&["-k", key_oid, "-o", output_file, "-i", input_file]);
    
    if hash {
        command.arg("-H");
    }
    
    let output = command.output().map_err(|e| format!("Failed to execute trustm_ecc_sign: {}", e))?;
    
    if output.status.success() {
        let file_contents = std::fs::read(output_file)
            .map_err(|e| format!("Failed to read the output file: {}", e))?;
        
        // let hash = extract_hash(&output.stdout)?;
        
        Ok((file_contents))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Failed to execute trustm_ecc_sign: {}", stderr))
    }
}

fn extract_hash(output: &[u8]) -> Result<Vec<u8>, String> {
    let lines = String::from_utf8_lossy(output);
    
    if let Some(hash_start) = lines.find("Hash Digest :") {
        let hash_start = hash_start + 14; // Length of "Hash Digest : " string
        let hash_end = lines.find("\n\n").unwrap_or(lines.len());
        let hash_text = lines[hash_start..hash_end].trim();
        
        let hash_bytes: Vec<u8> = hash_text
            .split_whitespace()
            .map(|byte| u8::from_str_radix(byte, 16))
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|e| format!("Failed to parse hash bytes: {}", e))?;
        
        Ok(hash_bytes)
    } else {
        Err("Hash digest not found in output".to_string())
    }
}

fn main() {
    let key_oid = "0xe0f3";
    let output_file = "testsignature.bin";
    let input_file = "helloworld.txt";
    let hash = true;
    
    match sign_file_with_key(key_oid, output_file, input_file, hash) {
        Ok((file_contents)) => {
            println!("Signature file generated successfully.");
        
            println!("File contents: {:?}", file_contents);
        },
        Err(err) => {
            println!("Signature file generation failed: {}", err);
        }
    }
}

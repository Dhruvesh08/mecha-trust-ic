use std::process::Command;

fn main() {
    let output = Command::new("/MECHA_TEST/optiga_trust_m/trustm_chipinfo")
        .output()
        .expect("Failed to execute the command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.split('\n').collect();

        for line in lines {
            if line.contains(":") {
                let parts: Vec<&str> = line.split(":").collect();
                let key = parts[0].trim();
                let value = parts[1].trim();
                println!("{}: {}", key, value);
            }
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command executed with error: {}", stderr);
    }
}


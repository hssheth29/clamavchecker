use std::process::Command;
use std::io::ErrorKind;

fn is_clamav_installed() -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg("clamscan --version")
        .output();

    match output {
        Ok(output) => output.status.success(),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                false
            } else {
                panic!("Failed to execute command: {}", e);
            }
        }
    }
}

fn install_clamav() {
    let mut command = Command::new("sh");
    command.arg("-c");

    let install_cmd = if cfg!(target_os = "redox") {
        "pkg install clamav"
    } else if cfg!(target_os = "linux") {
        let os_release = std::fs::read_to_string("/etc/os-release")
            .unwrap_or_else(|_| String::new());

        if os_release.contains("ID=debian") || os_release.contains("ID=ubuntu") {
            "apt-get update && apt-get install -y clamav"
        } else if os_release.contains("ID=fedora") || os_release.contains("ID=rhel") || os_release.contains("ID=centos") {
            "dnf install -y clamav || yum install -y clamav"
        } else {
            panic!("Unsupported Linux distribution.");
        }
    } else {
        panic!("Unsupported OS.");
    };

    command.arg(install_cmd);

    let output = command.output().expect("Failed to execute command");

    if !output.status.success() {
        panic!("Failed to install ClamAV: {:?}", output);
    }
}

fn main() {
    if is_clamav_installed() {
        println!("ClamAV is installed.");
    } else {
        println!("Installing ClamAV...");
        install_clamav();
        println!("ClamAV installation completed.");
    }
}
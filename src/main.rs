use os_info::Type;
use std::process::Command;

fn main() {
    // Check if ClamAV is installed
    if is_clamav_installed() {
        println!("ClamAV is already installed.");
    } else {
        // Identify the Linux distribution and install ClamAV accordingly
        let info = os_info::get();
        println!("OS information: {}", info);
        match info.os_type() {
            Type::Ubuntu | Type::Debian => {
                // For Debian-based systems
                run_command("sudo", &["apt-get", "update"]);
                run_command("sudo", &["apt-get", "install", "-y", "clamav"]);
            },
            Type::Fedora | Type::CentOS => {
                // For Fedora or CentOS
                run_command("sudo", &["dnf", "install", "-y", "clamav"]);
            },
            // Add more distributions as needed
            _ => println!("Unsupported Linux distribution for automated ClamAV installation."),
        }
    }
}

fn is_clamav_installed() -> bool {
    match Command::new("clamscan").arg("--version").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn run_command(command: &str, args: &[&str]) {
    match Command::new(command).args(args).status() {
        Ok(status) if status.success() => println!("ClamAV installed successfully."),
        _ => println!("Failed to execute command."),
    }
}

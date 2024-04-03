# ClamAV Installation Checker and Installer

This Rust program automates the process of checking if ClamAV, an open-source antivirus engine, is installed on your system. If ClamAV is not found, the program proceeds to install it. This tool supports various Linux distributions as well as Redox OS.

## Prerequisites

Before running this program, ensure you have Rust installed on your system. If Rust is not installed, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) to install Rust.

Additionally, the installation process requires superuser privileges. Ensure you have the necessary permissions to run commands with `sudo`.

## Installation

Clone this repository to your local machine using the following command:

```
git clone https://github.com/hssheth29/clamavchecker.git
```

Navigate to the cloned directory:

```
cd clamavchecker
```

Compile the program:

```
cargo build --release
```

## Usage

To run the program, navigate to the target release directory and execute the binary with superuser privileges:

```
sudo ./target/release/clamavchecker
```

### Important Reminder

This program requires sudo privileges to install ClamAV. A reminder is provided at the beginning of the installation process. Ensure you're prepared to authenticate as a superuser.

## Supported Platforms

- Linux (Debian, Ubuntu, Fedora, RHEL, CentOS)
- Redox OS

Other operating systems are not supported and will result in an error.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues to improve the program or extend support for additional operating systems.

## License

This project is licensed under the Apache 2.0 License - see the LICENSE file for details.

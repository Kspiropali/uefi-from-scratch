# uefi-from-scratch

uefi-from-scratch is a personal rust project to learn about UEFI and how to use it to boot a semi OS.

## Pre-requisites

- [Rust](https://www.rust-lang.org/tools/install)
- [QEMU](https://www.qemu.org/download/)
- [OVMF](https://www.linux-kvm.org/page/OVMF)
- [Rustup](https://rustup.rs/)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [just](https://crates.io/crates/just)

## Usage
#### Provided that the above pre-requisites are installed, run the following command to build and run the project:
```bash
just run
```
#### To clean the project, run the following command:
```bash
just clean
```

#### To run the project in debug mode, run the following command:
```bash
just debug
```

## Project Structure
##### The project structure is as follows but might change in the future:
```bash
├── .cargo/
│   └── config
├── .gitignore
├── emulated-firmware/
│   └── OVMF.fd
│   └── OVMF_VARS.fd
├── esp/
│   ├── EFI/
│   │   ├── BOOT/
│   │   │   ├── BOOTX64.EFI
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── enum.rs
├── target/
├── .github/
├── ACPI_Spec_6_5_Aug29.pdf
├── Cargo.lock
├── Cargo.toml
├── justfile
├── LICENSE
├── README.md
├── rust-toolchain.toml

```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
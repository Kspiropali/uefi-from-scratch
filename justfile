build:
    cargo build --release --target x86_64-unknown-uefi

build_debug:
    cargo build --target x86_64-unknown-uefi

test:
    cargo test --target x86_64-unknown-uefi

clean:
    cargo clean

run:
    cargo build --release --target x86_64-unknown-uefi && cp target/x86_64-unknown-uefi/release/uefi-from-scratch.efi esp/efi/boot/bootx64.efi && \
    qemu-system-x86_64 -drive if=pflash,format=raw,readonly=on,file=emulated-firmware/OVMF.fd -drive if=pflash,format=raw,readonly=on,file=emulated-firmware/OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp -net none -nographic -serial mon:stdio -m 64M

run-aarch64:
    cargo build --release --target aarch64-unknown-uefi && cp target/aarch64-unknown-uefi/release/uefi-from-scratch.efi esp/efi/boot/bootx64.efi &&
    qemu-system-x86_64 -drive if=pflash,format=raw,readonly=on,file=emulated-firmware/OVMF-aarch64.fd -drive if=pflash,format=raw,readonly=on,file=emulated-firmware/OVMF_VARS-aarch64.fd \
    -drive format=raw,file=fat:rw:esp -net none -nographic -serial mon:stdio -m 64M

run-i686:
    cargo build --release --target i686-unknown-uefi && cp target/i686-unknown-uefi/release/uefi-from-scratch.efi esp/efi/boot/bootx64.efi &&
    qemu-system-x86_64 -drive if=pflash,format=raw,readonly=on,file=emulated-firmware/OVMF-i686.fd -drive if=pflash,format=raw,readonly=on,file=emulated-firmware/OVMF_VARS-i686.fd \
    -drive format=raw,file=fat:rw:esp -net none -nographic -serial mon:stdio -m 64M

run_display:
    cargo build --release --target x86_64-unknown-uefi && cp target/x86_64-unknown-uefi/release/uefi-from-scratch.efi esp/efi/boot/bootx64.efi && \
    qemu-system-x86_64 -drive if=pflash,format=raw,readonly=on,file=emulated-firmware/OVMF.fd -drive if=pflash,format=raw,readonly=on,file=emulated-firmware/OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp -net none -m 64M
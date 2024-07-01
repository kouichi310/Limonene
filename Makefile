all: bootloader kernel

.PHONY: kernel
kernel: 
	${MAKE} -C $@ build

.PHONY: bootloader
bootloader:
	${MAKE} -C bootloader build

run: all
	script/run_qemu.sh bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi kernel/target/x86_64-unknown-limonene-elf/debug/kernel
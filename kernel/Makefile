build: 
	cargo +nightly build

run: build ../target/x86_64-unknown-uefi/debug/rikan.efi
	../script/run_qemu.sh
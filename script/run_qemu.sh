#!/bin/sh -ex

if [ $# -lt 1 ]
then
    cargo build
    EFI_FILE=target/x86_64-unknown-uefi/debug/Limonene.efi
else
    EFI_FILE=$1
fi

DEVENV_DIR=$(dirname "$0")
ANOTHER_FILE=$2
DISK_IMG=./disk.img
MOUNT_POINT=./mnt

$DEVENV_DIR/make_image.sh $DISK_IMG $MOUNT_POINT $EFI_FILE $ANOTHER_FILE
$DEVENV_DIR/run_image.sh $DISK_IMG

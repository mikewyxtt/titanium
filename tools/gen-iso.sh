#!/usr/bin/env bash


set -e

if [ ! -d ./target/STAGING ]; then
    echo 'titanium must be built before generating an iso. Exiting.'
    exit 1
fi

TEMPDIR=$(mktemp -d)

mkdir -pv $TEMPDIR/boot/grub
cp -v ./target/STAGING/bin/kernel.elf $TEMPDIR/kernel.elf
cp -v ./target/x86_64-sel4/debug/root_task.elf $TEMPDIR/initrd.img

## Create grub.cfg
cat <<EOF > $TEMPDIR/boot/grub/grub.cfg
# Set the timeout to 5 seconds
set timeout=5

# Set the default boot entry to 0 (first entry in the menu)
set default=0

# Define a menu entry for booting a Linux kernel
menuentry "Titanium Kernel" {
    multiboot2 /kernel.elf
    module2 /initrd.img
}

EOF

grub-mkrescue -o ./target/titanium.iso $TEMPDIR

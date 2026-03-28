#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# --- Clone upstream kernel source, or pull if already present ---
if [ -d kernel/.git ]; then
  git -C kernel pull
else
  git clone --depth 1 -b chromeos-6.6 \
    https://chromium.googlesource.com/chromiumos/third_party/kernel \
    kernel
fi

# --- Install build dependencies ---
sudo apt-get update
sudo apt-get install -y \
  build-essential bc kmod cpio flex libncurses5-dev \
  libelf-dev libssl-dev dwarves bison \
  libguestfs-tools

# --- Configure kernel ---
cd kernel
CHROMEOS_KERNEL_FAMILY=termina ./chromeos/scripts/prepareconfig container-vm-x86_64

# -- Initrd support --
scripts/config --enable CONFIG_BLK_DEV_INITRD
scripts/config --enable CONFIG_RD_GZIP
scripts/config --enable CONFIG_RD_BZIP2
scripts/config --enable CONFIG_RD_LZMA
scripts/config --enable CONFIG_RD_XZ
scripts/config --enable CONFIG_RD_LZO
scripts/config --enable CONFIG_RD_LZ4
scripts/config --enable CONFIG_RD_ZSTD

# -- Ensure jail-VM essentials are enabled --
scripts/config --enable CONFIG_VIRTIO_PCI
scripts/config --enable CONFIG_VIRTIO_MMIO
scripts/config --enable CONFIG_VIRTIO_BLK
scripts/config --enable CONFIG_VIRTIO_NET
scripts/config --enable CONFIG_VIRTIO_CONSOLE
scripts/config --enable CONFIG_VSOCKETS
scripts/config --enable CONFIG_VIRTIO_VSOCKETS
scripts/config --enable CONFIG_NET_9P
scripts/config --enable CONFIG_NET_9P_VIRTIO
scripts/config --enable CONFIG_9P_FS
scripts/config --enable CONFIG_SERIAL_8250
scripts/config --enable CONFIG_SERIAL_8250_CONSOLE
scripts/config --enable CONFIG_DEVTMPFS
scripts/config --enable CONFIG_DEVTMPFS_MOUNT
scripts/config --enable CONFIG_TMPFS
scripts/config --enable CONFIG_PROC_FS
scripts/config --enable CONFIG_SYSFS
scripts/config --enable CONFIG_BINFMT_ELF
scripts/config --enable CONFIG_FTRACE
scripts/config --enable CONFIG_FTRACE_SYSCALLS

# -- Disable desktop/server subsystems unnecessary for VM jails --
scripts/config --disable CONFIG_USB_SUPPORT
scripts/config --disable CONFIG_SOUND
scripts/config --disable CONFIG_HID_SUPPORT
scripts/config --disable CONFIG_MEDIA_SUPPORT
scripts/config --disable CONFIG_BT
scripts/config --disable CONFIG_DRM
scripts/config --disable CONFIG_INPUT_KEYBOARD
scripts/config --disable CONFIG_INPUT_MOUSE
scripts/config --disable CONFIG_INPUT_JOYSTICK
scripts/config --disable CONFIG_INPUT_TABLET
scripts/config --disable CONFIG_INPUT_TOUCHSCREEN
scripts/config --disable CONFIG_WLAN
scripts/config --disable CONFIG_WIRELESS
scripts/config --disable CONFIG_CFG80211
scripts/config --disable CONFIG_RFKILL
scripts/config --disable CONFIG_NFC
scripts/config --disable CONFIG_PCCARD
scripts/config --disable CONFIG_PARPORT
scripts/config --disable CONFIG_GAMEPORT
scripts/config --disable CONFIG_HWMON
scripts/config --disable CONFIG_THERMAL
scripts/config --disable CONFIG_AGP
scripts/config --disable CONFIG_VGA_ARB
scripts/config --disable CONFIG_FB
scripts/config --disable CONFIG_FRAMEBUFFER_CONSOLE

make olddefconfig

# --- Build bzImage ---
make -j"$(nproc)" bzImage

# --- Collect metadata ---
KERNEL_RELEASE=$(make -s kernelrelease)
COMMIT_SHA=$(git rev-parse HEAD)
SHORT_SHA=$(git rev-parse --short HEAD)
TARBALL="oklinux-kernel-x86_64-${KERNEL_RELEASE}-${COMMIT_SHA}.tar.gz"

cd "$SCRIPT_DIR"

# --- Package kernel ---
tar czf "${TARBALL}" -C kernel arch/x86/boot/bzImage
sha256sum "${TARBALL}" | tee "${TARBALL}.sha256"

# --- Export metadata for CI consumption ---
if [ -n "${GITHUB_OUTPUT:-}" ]; then
  echo "kernel_release=${KERNEL_RELEASE}" >> "$GITHUB_OUTPUT"
  echo "commit_sha=${COMMIT_SHA}"         >> "$GITHUB_OUTPUT"
  echo "short_sha=${SHORT_SHA}"            >> "$GITHUB_OUTPUT"
  echo "tarball_name=${TARBALL}"            >> "$GITHUB_OUTPUT"
fi

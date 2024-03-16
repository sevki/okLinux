#!/bin/bash

# Read kernel version from kernel-version.txx to KERNEL_VERSION
KERNEL_VERSION=$(cat kernel-version.txt)

# Update the package list
apt-get update -y

# Install build dependencies
apt-get install \
    libguestfs-tools \
    -y

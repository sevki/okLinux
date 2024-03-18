#!/bin/bash
set -e
# Download the kernel
curl -L  -o kernel-x86_64-9c0a3eb26a6d8cbfdbd62bf0bc3dc2ebd2a0655c.tar.gz
# Verify the hash
echo d11ccefe44c3cd6149355fcdf3aba186022fe4fb27cb2df2f5e0c2d92aecaed3  kernel-x86_64-9c0a3eb26a6d8cbfdbd62bf0bc3dc2ebd2a0655c.tar.gz | sha256sum -c -
		
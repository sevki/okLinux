#!/bin/bash
set -e
# Copyright oknotok.computer (c) 2024
# All rights reserved
# Licensed under the BSD 3-Clause License

# Download the kernel
curl -L https://github.com/sevki/kernel/actions/runs/8336089767/artifacts/1337296962 -o kernel-x86_64-exp-b91c53218e9b1dae75a7da35117897ef16114874.tar.gz
# Verify the hash
echo 73cb5b3e27e293cb312d30f31efd86090e273470f6abad1874e9ca4f7b2e322f  kernel-x86_64-exp-b91c53218e9b1dae75a7da35117897ef16114874.tar.gz | sha256sum -c -


#!/bin/bash
set -e
# Copyright oknotok.computer (c) 2024
# All rights reserved
# Licensed under the BSD 3-Clause License

# Download the kernel
curl -L https://github.com/sevki/kernel/actions/runs/8336116865/artifacts/1337302799 -o kernel-x86_64-6.6.22-e4d92a0cf76fdbd3dbeae1394fc9d3033b683fb7.tar.gz
# Verify the hash
echo d68ea7684973c4a0666815138f0cdaeab7b1fc5a4730acb6e29c942e2bf1186f  kernel-x86_64-6.6.22-e4d92a0cf76fdbd3dbeae1394fc9d3033b683fb7.tar.gz | sha256sum -c -


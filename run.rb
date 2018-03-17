#!/bin/ruby

if system "bootimage --target x86_64-reduk"
    exec "qemu-system-x86_64 -drive format=raw,file=bootimage.bin"
end
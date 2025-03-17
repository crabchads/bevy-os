#!/usr/bin/env brush

# TODO: reindeer doesn't seem to recognize "default-features = false" 
# reindeer --third-party-dir third_party buckify
# buck2 build //:bevy-os --out=bevy-os
cargo rustc --target aarch64-unknown-none -- -Clink-arg=-Tsrc/image.ld
if ! [ $? -eq 0 ]; then
    echo "Cargo failed to run"
    exit 1
fi

qemu-system-aarch64 \
    -machine virt,gic-version=3 \
    -cpu cortex-a72 \
    -accel tcg,thread=multi \
    -m 128M \
    -smp 4 \
    -d cpu_reset \
    -device virtio-gpu-pci \
    -serial mon:stdio \
    -kernel target/aarch64-unknown-none/debug/bevy-os

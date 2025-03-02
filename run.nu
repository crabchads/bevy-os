# ../reindeer/target/debug/reindeer --third-party-dir third_party buckify
# buck2 build //:bevy-os --out=bevy-os
cargo rustc --target aarch64-unknown-none-softfloat -- -Clink-arg=-Tsrc/linker.ld

(qemu-system-aarch64
    -machine virt
    -accel tcg,thread=multi
    -m 1024M
    -smp 4
    -device virtio-gpu-pci
    -serial mon:stdio
    -kernel target/aarch64-unknown-none-softfloat/debug/bevy-os
)

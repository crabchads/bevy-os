rust_binary(
    name = "bevy-os",
    srcs = ["src/main.rs"],
    crate_root = "src/main.rs",
    deps = [
        "//third_party:bevy",
    ],
    rustc_flags = [
        "-Cpanic=abort",
    ],
    default_target_platform = "root//platforms:baremetal-arm64",
    target_compatible_with = ["root//platforms:baremetal-arm64"],
)

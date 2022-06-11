load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@io_bazel_rules_docker//rust:image.bzl", "rust_image")

rust_image(
    name = "chibiri9cc_server",
    srcs = ["src/main.rs"],
    aliases = aliases(),
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    deps = [
        "//chibiri9cc_lib",
    ] + all_crate_deps(
        normal = True,
    ),
)

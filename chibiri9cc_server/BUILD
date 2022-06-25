load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@io_bazel_rules_docker//container:container.bzl", "container_image")
load("@io_bazel_rules_docker//rust:image.bzl", "rust_image")

rust_image(
    name = "chibiri9cc_server_image",
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

container_image(
    name = "chibiri9cc_server",
    base = ":chibiri9cc_server_image",
    ports = ["3000"],
)

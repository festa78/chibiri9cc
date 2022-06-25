load("@io_bazel_rules_docker//go:image.bzl", "go_image")
load("@io_bazel_rules_docker//container:container.bzl", "container_image")

go_image(
    name = "go_server_image",
    srcs = ["src/main.go"],
)

container_image(
    name = "go_server",
    base = ":go_server_image",
    ports = ["8080"],
)

load("@io_bazel_rules_docker//go:image.bzl", "go_image")
load("@io_bazel_rules_docker//container:container.bzl", "container_image")

container_image(
    name = "go_server_image",
    base = "@golang_image_dockerfile//image:dockerfile_image.tar",
    ports = ["8080"],
)

go_image(
    name = "go_server",
    srcs = ["src/main.go"],
    base = ":go_server_image",
)
